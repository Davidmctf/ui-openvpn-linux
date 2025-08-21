use std::process::Command;
use warp::{Filter, Rejection, Reply, Buf};
use warp::multipart::{FormData, Part};
use futures_util::{TryStreamExt, StreamExt};

pub async fn run_web_gui() {
    println!("🌐 Iniciando interfaz web en http://localhost:8081");
    
    // Ruta principal - HTML de la interfaz
    let index = warp::path::end().map(|| {
        warp::reply::html(HTML_INTERFACE)
    });
    
    // Servir assets estáticos
    let logo = warp::path!("assets" / "logo.png")
        .map(|| {
            // Servir el logo desde los assets copiados
            warp::reply::with_header(
                &include_bytes!("../../assets/2261aaaa-bad7-4426-8cc1-f93cd6c4c067.png")[..],
                "content-type",
                "image/png",
            )
        });

    let favicon = warp::path("favicon.ico")
        .map(|| {
            // Servir el favicon
            warp::reply::with_header(
                &include_bytes!("../../assets/openvpn-manager-logo.png")[..],
                "content-type",
                "image/x-icon",
            )
        });
    
    // API para listar VPNs
    let api_list = warp::path!("api" / "vpns")
        .and(warp::get())
        .map(|| {
            let vpns = get_vpn_list();
            warp::reply::json(&vpns)
        });
    
    // API para obtener estado
    let api_status = warp::path!("api" / "status")
        .and(warp::get())
        .map(|| {
            let status = get_vpn_status();
            warp::reply::json(&status)
        });
    
    // API para conectar
    let api_connect = warp::path!("api" / "connect")
        .and(warp::post())
        .and(warp::body::json())
        .map(|req: ConnectRequest| {
            let result = connect_vpn(&req.name);
            warp::reply::json(&result)
        });
    
    // API para desconectar
    let api_disconnect = warp::path!("api" / "disconnect")
        .and(warp::post())
        .map(|| {
            let result = disconnect_vpn();
            warp::reply::json(&result)
        });
    
    // API para subir archivo VPN
    let api_upload = warp::path!("api" / "upload")
        .and(warp::post())
        .and(warp::multipart::form().max_length(10 * 1024 * 1024)) // Max 10MB - reducir el límite
        .and_then(handle_file_upload);
    
    let routes = index
        .or(logo)
        .or(favicon)
        .or(api_list)
        .or(api_status)
        .or(api_connect)
        .or(api_disconnect)
        .or(api_upload);
    
    // Abrir navegador automáticamente
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        let _ = Command::new("xdg-open")
            .arg("http://localhost:8081")
            .spawn();
    });
    
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8081))
        .await;
}

async fn handle_file_upload(mut form: FormData) -> Result<impl Reply, Rejection> {
    println!("🔄 Iniciando upload de archivo...");
    
    // Procesar partes una por una en lugar de recolectar todas
    while let Some(part_result) = form.next().await {
        let p = match part_result {
            Ok(part) => part,
            Err(e) => {
                println!("❌ Error obteniendo parte del form: {:?}", e);
                return Ok(warp::reply::json(&ApiResponse {
                    success: false,
                    message: "Error procesando archivo".to_string(),
                }));
            }
        };
        
        if p.name() == "vpn_file" {
            let filename = p.filename().unwrap_or("unknown.ovpn").to_string();
            println!("📄 Archivo detectado: {}", filename);
            
            if !filename.ends_with(".ovpn") {
                println!("❌ Archivo rechazado: no es .ovpn");
                return Ok(warp::reply::json(&ApiResponse {
                    success: false,
                    message: format!("Solo se permiten archivos .ovpn. Recibido: {}", filename),
                }));
            }
            
            let bytes = p.stream().try_fold(Vec::new(), |mut vec, data| {
                vec.extend_from_slice(data.chunk());
                async move { Ok(vec) }
            }).await.map_err(|e| {
                println!("❌ Error leyendo stream del archivo: {:?}", e);
                warp::reject::reject()
            })?;
            
            println!("📊 Archivo leído: {} bytes", bytes.len());
            
            // Guardar archivo en directorio home/.vpn-configs/
            let home = std::env::var("HOME").unwrap_or("/tmp".to_string());
            let vpn_dir = format!("{}/.vpn-configs", home);
            
            println!("📂 Creando directorio: {}", vpn_dir);
            std::fs::create_dir_all(&vpn_dir).map_err(|e| {
                println!("❌ Error creando directorio {}: {:?}", vpn_dir, e);
                warp::reject::reject()
            })?;
            
            let file_path = format!("{}/{}", vpn_dir, filename);
            println!("💾 Guardando archivo en: {}", file_path);
            
            std::fs::write(&file_path, &bytes).map_err(|e| {
                println!("❌ Error escribiendo archivo {}: {:?}", file_path, e);
                warp::reject::reject()
            })?;
            
            println!("✅ Archivo guardado exitosamente");
            
            // Importar con nmcli
            println!("🔗 Importando con nmcli: {}", file_path);
            let result = Command::new("nmcli")
                .args(&["connection", "import", "type", "openvpn", "file", &file_path])
                .output();
                
            match result {
                Ok(output) if output.status.success() => {
                    println!("✅ Importación exitosa con nmcli");
                    return Ok(warp::reply::json(&ApiResponse {
                        success: true,
                        message: format!("✅ Archivo {} importado correctamente", filename),
                    }));
                }
                Ok(output) => {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    println!("❌ Error en nmcli - stdout: {}, stderr: {}", stdout, stderr);
                    return Ok(warp::reply::json(&ApiResponse {
                        success: false,
                        message: format!("❌ Error importando: {}", stderr),
                    }));
                }
                Err(e) => {
                    println!("❌ Error ejecutando nmcli: {:?}", e);
                    return Ok(warp::reply::json(&ApiResponse {
                        success: false,
                        message: format!("❌ Error ejecutando nmcli: {}", e),
                    }));
                }
            }
            
            // Si llegamos aquí, procesamos exitosamente el archivo
            return Ok(warp::reply::json(&ApiResponse {
                success: true,
                message: "Archivo procesado".to_string(),
            }));
        }
    }
    
    println!("❌ No se encontró archivo con name='vpn_file'");
    Ok(warp::reply::json(&ApiResponse {
        success: false,
        message: "No se encontró archivo VPN en el formulario".to_string(),
    }))
}

#[derive(serde::Deserialize)]
struct ConnectRequest {
    name: String,
}

#[derive(serde::Serialize)]
struct VpnInfo {
    name: String,
    status: String,
}

#[derive(serde::Serialize)]
struct StatusInfo {
    connected: bool,
    active_vpn: Option<String>,
}

#[derive(serde::Serialize)]
struct ApiResponse {
    success: bool,
    message: String,
}

fn get_vpn_list() -> Vec<VpnInfo> {
    let mut vpns = Vec::new();
    
    if let Ok(output) = Command::new("nmcli")
        .args(&["connection", "show"])
        .output() 
    {
        let connections = String::from_utf8_lossy(&output.stdout);
        for line in connections.lines().skip(1) {
            if !line.trim().is_empty() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 && parts[2] == "vpn" {
                    vpns.push(VpnInfo {
                        name: parts[0].to_string(),
                        status: "available".to_string(),
                    });
                }
            }
        }
    }
    
    vpns
}

fn get_vpn_status() -> StatusInfo {
    if let Ok(output) = Command::new("nmcli")
        .args(&["connection", "show", "--active"])
        .output()
    {
        let connections = String::from_utf8_lossy(&output.stdout);
        for line in connections.lines().skip(1) {
            if !line.trim().is_empty() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 && parts[2] == "vpn" {
                    return StatusInfo {
                        connected: true,
                        active_vpn: Some(parts[0].to_string()),
                    };
                }
            }
        }
    }
    
    StatusInfo {
        connected: false,
        active_vpn: None,
    }
}

fn connect_vpn(name: &str) -> ApiResponse {
    match Command::new("nmcli")
        .args(&["connection", "up", name])
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                ApiResponse {
                    success: true,
                    message: format!("Conectado a {}", name),
                }
            } else {
                ApiResponse {
                    success: false,
                    message: "Error al conectar".to_string(),
                }
            }
        }
        Err(_) => ApiResponse {
            success: false,
            message: "Error ejecutando comando".to_string(),
        }
    }
}

fn disconnect_vpn() -> ApiResponse {
    if let Ok(output) = Command::new("nmcli")
        .args(&["connection", "show", "--active"])
        .output()
    {
        let connections = String::from_utf8_lossy(&output.stdout);
        for line in connections.lines().skip(1) {
            if !line.trim().is_empty() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 && parts[2] == "vpn" {
                    let _ = Command::new("nmcli")
                        .args(&["connection", "down", parts[0]])
                        .output();
                    return ApiResponse {
                        success: true,
                        message: format!("Desconectado de {}", parts[0]),
                    };
                }
            }
        }
    }
    
    ApiResponse {
        success: true,
        message: "No había conexiones activas".to_string(),
    }
}

const HTML_INTERFACE: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <title>🚀 UI OpenVPN Linux - Navegador Espacial</title>
    <meta charset="UTF-8">
    <link rel="icon" type="image/x-icon" href="/favicon.ico">
    <style>
        body { 
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; 
            max-width: 1000px; 
            margin: 0 auto; 
            padding: 20px;
            background: linear-gradient(135deg, #0c0c0c 0%, #1a1a2e 50%, #16213e 100%);
            color: #ffffff;
            min-height: 100vh;
        }
        .container {
            background: rgba(255, 255, 255, 0.05);
            backdrop-filter: blur(10px);
            padding: 30px;
            border-radius: 20px;
            border: 1px solid rgba(255, 255, 255, 0.1);
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
        }
        .header {
            text-align: center;
            margin-bottom: 30px;
        }
        .logo {
            width: 100px;
            height: 100px;
            margin: 0 auto 20px;
            display: block;
            filter: drop-shadow(0 0 20px rgba(0, 150, 255, 0.3));
        }
        h1 { 
            color: #00d4ff; 
            text-align: center;
            font-size: 2.5em;
            margin: 0;
            text-shadow: 0 0 10px rgba(0, 212, 255, 0.5);
        }
        .subtitle {
            color: #888;
            text-align: center;
            font-size: 1.1em;
            margin-top: 10px;
        }
        .status { 
            padding: 15px; 
            border-radius: 15px; 
            margin: 20px 0;
            text-align: center;
            font-weight: bold;
            font-size: 1.2em;
            border: 2px solid transparent;
        }
        .connected { 
            background: linear-gradient(45deg, #00ff88, #00cc66);
            color: #003d1a;
            border-color: #00ff88;
            box-shadow: 0 0 20px rgba(0, 255, 136, 0.3);
        }
        .disconnected { 
            background: linear-gradient(45deg, #ff4757, #ff3742);
            color: #fff;
            border-color: #ff4757;
            box-shadow: 0 0 20px rgba(255, 71, 87, 0.3);
        }
        .vpn-list { 
            border: 1px solid rgba(0, 212, 255, 0.3); 
            border-radius: 15px;
            max-height: 300px;
            overflow-y: auto;
            background: rgba(0, 0, 0, 0.2);
            backdrop-filter: blur(5px);
        }
        .vpn-item { 
            padding: 15px; 
            border-bottom: 1px solid rgba(255, 255, 255, 0.1);
            cursor: pointer;
            transition: all 0.3s ease;
            font-size: 1.1em;
        }
        .vpn-item:hover { 
            background: rgba(0, 212, 255, 0.1);
            transform: translateX(5px);
        }
        .vpn-item.selected { 
            background: linear-gradient(45deg, #00d4ff, #0099cc);
            color: white;
            box-shadow: 0 0 15px rgba(0, 212, 255, 0.4);
        }
        .buttons {
            display: flex;
            gap: 15px;
            justify-content: center;
            margin: 30px 0;
            flex-wrap: wrap;
        }
        button { 
            padding: 12px 24px; 
            border: 2px solid transparent; 
            border-radius: 25px;
            cursor: pointer;
            font-size: 16px;
            font-weight: bold;
            transition: all 0.3s ease;
            text-shadow: 0 0 5px rgba(0,0,0,0.5);
            position: relative;
            overflow: hidden;
        }
        .btn-primary { 
            background: linear-gradient(45deg, #00d4ff, #0099cc);
            color: white;
            border-color: #00d4ff;
            box-shadow: 0 4px 15px rgba(0, 212, 255, 0.4);
        }
        .btn-success { 
            background: linear-gradient(45deg, #00ff88, #00cc66);
            color: #003d1a;
            border-color: #00ff88;
            box-shadow: 0 4px 15px rgba(0, 255, 136, 0.4);
        }
        .btn-danger { 
            background: linear-gradient(45deg, #ff4757, #ff3742);
            color: white;
            border-color: #ff4757;
            box-shadow: 0 4px 15px rgba(255, 71, 87, 0.4);
        }
        .btn-secondary { 
            background: linear-gradient(45deg, #747d8c, #57606f);
            color: white;
            border-color: #747d8c;
            box-shadow: 0 4px 15px rgba(116, 125, 140, 0.4);
        }
        button:hover {
            transform: translateY(-2px);
            box-shadow: 0 6px 20px rgba(0,0,0,0.3);
        }
        button:disabled { 
            opacity: 0.5; 
            cursor: not-allowed; 
            transform: none;
        }
        .message { 
            padding: 15px; 
            margin: 15px 0;
            border-radius: 10px;
            font-weight: bold;
            text-align: center;
        }
        .success { 
            background: linear-gradient(45deg, #00ff88, #00cc66);
            color: #003d1a;
            box-shadow: 0 0 15px rgba(0, 255, 136, 0.3);
        }
        .error { 
            background: linear-gradient(45deg, #ff4757, #ff3742);
            color: #fff;
            box-shadow: 0 0 15px rgba(255, 71, 87, 0.3);
        }
        .upload-section {
            background: rgba(0, 0, 0, 0.3);
            border-radius: 15px;
            padding: 20px;
            margin: 20px 0;
            border: 1px solid rgba(0, 212, 255, 0.3);
        }
        .upload-area {
            border: 2px dashed #00d4ff;
            border-radius: 10px;
            padding: 30px;
            text-align: center;
            cursor: pointer;
            transition: all 0.3s ease;
            background: rgba(0, 212, 255, 0.05);
        }
        .upload-area:hover {
            background: rgba(0, 212, 255, 0.1);
            transform: scale(1.02);
        }
        .upload-area.dragover {
            border-color: #00ff88;
            background: rgba(0, 255, 136, 0.1);
        }
        input[type="file"] {
            display: none;
        }
        h3 {
            color: #00d4ff;
            margin-bottom: 15px;
            text-shadow: 0 0 5px rgba(0, 212, 255, 0.5);
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <img src="/assets/logo.png" alt="Astronauta VPN" class="logo">
            <h1>UI OpenVPN Linux</h1>
            <div class="subtitle">🚀 Navegador Espacial de Conexiones VPN ✨</div>
        </div>
        
        <div id="status" class="status disconnected">
            🔴 Desconectado del Ciberespacio
        </div>
        
        <div id="message"></div>
        
        <div class="upload-section">
            <h3>🛸 Importar Nueva Estación VPN</h3>
            <div class="upload-area" id="upload-area">
                <div style="font-size: 3em; margin-bottom: 10px;">📡</div>
                <div><strong>Arrastra tu archivo .ovpn aquí</strong></div>
                <div style="margin: 10px 0; color: #888;">o haz click para seleccionar</div>
                <input type="file" id="file-input" accept=".ovpn" multiple>
            </div>
        </div>
        
        <h3>🌌 Estaciones VPN Disponibles</h3>
        <div id="vpn-list" class="vpn-list">
            <div style="padding: 20px; text-align: center; color: #888;">
                🔄 Explorando el ciberespacio...
            </div>
        </div>
        
        <div class="buttons">
            <button id="refresh-btn" class="btn-secondary">🔄 Explorar Estaciones</button>
            <button id="connect-btn" class="btn-success" disabled>🚀 Conectar</button>
            <button id="disconnect-btn" class="btn-danger">🛑 Desconectar</button>
        </div>
    </div>

    <script>
        let selectedVpn = null;
        
        async function loadVpns() {
            try {
                const response = await fetch('/api/vpns');
                const vpns = await response.json();
                
                const listDiv = document.getElementById('vpn-list');
                if (vpns.length === 0) {
                    listDiv.innerHTML = '<div style="padding: 20px; text-align: center; color: #888;">🌌 No hay estaciones disponibles<br><small>Importa un archivo .ovpn para comenzar</small></div>';
                    return;
                }
                
                listDiv.innerHTML = vpns.map(vpn => 
                    `<div class="vpn-item" onclick="selectVpn('${vpn.name}')">
                        🛰️ ${vpn.name}
                    </div>`
                ).join('');
            } catch (error) {
                showMessage('Error cargando VPNs', 'error');
            }
        }
        
        async function loadStatus() {
            try {
                const response = await fetch('/api/status');
                const status = await response.json();
                
                const statusDiv = document.getElementById('status');
                if (status.connected) {
                    statusDiv.className = 'status connected';
                    statusDiv.innerHTML = `🚀 Navegando desde estación: ${status.active_vpn}`;
                } else {
                    statusDiv.className = 'status disconnected';
                    statusDiv.innerHTML = '🔴 Desconectado del Ciberespacio';
                }
            } catch (error) {
                showMessage('Error obteniendo estado', 'error');
            }
        }
        
        function selectVpn(name) {
            selectedVpn = name;
            document.querySelectorAll('.vpn-item').forEach(item => {
                item.classList.remove('selected');
            });
            event.target.classList.add('selected');
            document.getElementById('connect-btn').disabled = false;
        }
        
        async function connectVpn() {
            if (!selectedVpn) return;
            
            try {
                const response = await fetch('/api/connect', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ name: selectedVpn })
                });
                const result = await response.json();
                
                showMessage(result.message, result.success ? 'success' : 'error');
                setTimeout(() => loadStatus(), 1000);
            } catch (error) {
                showMessage('Error conectando', 'error');
            }
        }
        
        async function disconnectVpn() {
            try {
                const response = await fetch('/api/disconnect', { method: 'POST' });
                const result = await response.json();
                
                showMessage(result.message, result.success ? 'success' : 'error');
                setTimeout(() => loadStatus(), 1000);
            } catch (error) {
                showMessage('Error desconectando', 'error');
            }
        }
        
        function showMessage(text, type) {
            const messageDiv = document.getElementById('message');
            messageDiv.className = `message ${type}`;
            messageDiv.textContent = text;
            setTimeout(() => messageDiv.innerHTML = '', 3000);
        }
        
        function refresh() {
            loadVpns();
            loadStatus();
        }
        
        // Upload functionality
        const uploadArea = document.getElementById('upload-area');
        const fileInput = document.getElementById('file-input');
        
        uploadArea.onclick = () => fileInput.click();
        
        // Drag & Drop
        uploadArea.ondragover = (e) => {
            e.preventDefault();
            uploadArea.classList.add('dragover');
        };
        
        uploadArea.ondragleave = () => {
            uploadArea.classList.remove('dragover');
        };
        
        uploadArea.ondrop = (e) => {
            e.preventDefault();
            uploadArea.classList.remove('dragover');
            handleFiles(e.dataTransfer.files);
        };
        
        fileInput.onchange = (e) => {
            handleFiles(e.target.files);
        };
        
        async function handleFiles(files) {
            for (let file of files) {
                if (file.name.endsWith('.ovpn')) {
                    await uploadFile(file);
                } else {
                    showMessage(`❌ ${file.name} no es un archivo .ovpn válido`, 'error');
                }
            }
        }
        
        async function uploadFile(file) {
            const formData = new FormData();
            formData.append('vpn_file', file);
            
            try {
                showMessage(`🚀 Subiendo ${file.name}...`, 'success');
                
                const response = await fetch('/api/upload', {
                    method: 'POST',
                    body: formData
                });
                
                const result = await response.json();
                showMessage(result.message, result.success ? 'success' : 'error');
                
                if (result.success) {
                    setTimeout(() => {
                        loadVpns();
                    }, 1000);
                }
            } catch (error) {
                showMessage(`❌ Error subiendo ${file.name}`, 'error');
            }
        }

        // Event listeners
        document.getElementById('refresh-btn').onclick = refresh;
        document.getElementById('connect-btn').onclick = connectVpn;
        document.getElementById('disconnect-btn').onclick = disconnectVpn;
        
        // Cargar datos iniciales
        refresh();
        
        // Auto-refresh cada 5 segundos
        setInterval(loadStatus, 5000);
    </script>
</body>
</html>
"#;