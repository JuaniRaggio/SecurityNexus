# Deployment Guide - DigitalOcean

Guía completa para deployar Polkadot Security Nexus en DigitalOcean.

## Tabla de Contenidos

1. [Servicios Requeridos](#servicios-requeridos)
2. [Setup Inicial](#setup-inicial)
3. [Configuración del Servidor](#configuración-del-servidor)
4. [Deployment del Dashboard](#deployment-del-dashboard)
5. [Deployment de SAFT Enhanced](#deployment-de-saft-enhanced)
6. [Nginx y SSL](#nginx-y-ssl)
7. [Monitoring](#monitoring)
8. [Troubleshooting](#troubleshooting)

---

## Servicios Requeridos

### Fase 1: Demo/Pitch (Actual)

#### 1. Droplet (VPS)
- **Plan recomendado**: Basic Droplet - $12/mes
- **Specs**: 2 vCPUs, 2GB RAM, 50GB SSD
- **OS**: Ubuntu 24.04 LTS
- **Datacenter**: New York 1 (NYC1) o el más cercano a tu audiencia

#### 2. Dominio (Opcional pero recomendado)
- **Costo**: ~$12/año
- **Ejemplo**: `securitynexus.xyz` o `polkadot-security.dev`
- **Alternativa gratis**: Usar IP del droplet

#### 3. Firewall
- **Costo**: Gratis
- **Configuración**: SSH (22), HTTP (80), HTTPS (443)

### Fase 2: Producción (Futuro - 3-6 meses)

#### 4. Managed PostgreSQL
- **Plan**: Basic - $15/mes
- **Specs**: 1GB RAM, 10GB storage
- **Para**: Almacenar auditorías, reportes, usuarios

#### 5. Load Balancer
- **Costo**: $12/mes
- **Para**: Alta disponibilidad con múltiples droplets

#### 6. Spaces (Object Storage)
- **Costo**: $5/mes (250GB incluidos)
- **Para**: Assets estáticos, reportes HTML de SAFT

---

## Setup Inicial

### 1. Crear Cuenta en DigitalOcean

```bash
# Obtén $200 en créditos gratis:
# https://try.digitalocean.com/freetrialoffer/
```

### 2. Crear Droplet

**Vía Web UI:**
1. Click en "Create" → "Droplets"
2. Selecciona:
   - **Image**: Ubuntu 24.04 LTS
   - **Plan**: Basic - $12/mes (2GB RAM)
   - **Datacenter**: NYC1 o el más cercano
   - **Authentication**: SSH Key (crear si no tienes)
   - **Hostname**: `security-nexus-demo`
3. Click "Create Droplet"

**Vía CLI (doctl):**
```bash
# Instalar doctl
brew install doctl  # macOS
# o
snap install doctl  # Linux

# Autenticar
doctl auth init

# Crear SSH key
ssh-keygen -t ed25519 -C "security-nexus-deploy"

# Subir SSH key a DigitalOcean
doctl compute ssh-key import security-nexus-key \
  --public-key-file ~/.ssh/id_ed25519.pub

# Crear droplet
doctl compute droplet create security-nexus-demo \
  --region nyc1 \
  --size s-2vcpu-2gb \
  --image ubuntu-24-04-x64 \
  --ssh-keys $(doctl compute ssh-key list --format ID --no-header)
```

### 3. Configurar Firewall

```bash
# Vía doctl
doctl compute firewall create \
  --name security-nexus-fw \
  --inbound-rules "protocol:tcp,ports:22,sources:addresses:0.0.0.0/0,sources:addresses:::/0 protocol:tcp,ports:80,sources:addresses:0.0.0.0/0,sources:addresses:::/0 protocol:tcp,ports:443,sources:addresses:0.0.0.0/0,sources:addresses:::/0" \
  --droplet-ids $(doctl compute droplet list --format ID --no-header)
```

---

## Configuración del Servidor

### 1. Conectar al Droplet

```bash
# Obtener IP del droplet
doctl compute droplet list

# Conectar vía SSH
ssh root@YOUR_DROPLET_IP
```

### 2. Setup Inicial del Servidor

```bash
# Actualizar sistema
apt update && apt upgrade -y

# Instalar dependencias base
apt install -y \
  curl \
  git \
  build-essential \
  pkg-config \
  libssl-dev \
  nginx \
  certbot \
  python3-certbot-nginx

# Crear usuario para la aplicación
adduser --disabled-password --gecos "" nexus
usermod -aG sudo nexus

# Cambiar a usuario nexus
su - nexus
```

### 3. Instalar Rust (como usuario nexus)

```bash
# Instalar rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# Instalar targets necesarios
rustup target add wasm32-unknown-unknown
rustup component add rust-src

# Verificar
rustc --version
cargo --version
```

### 4. Instalar Node.js y pnpm

```bash
# Instalar Node.js 20 LTS
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs

# Instalar pnpm
npm install -g pnpm

# Verificar
node --version
pnpm --version
```

### 5. Clonar Repositorio

```bash
# Como usuario nexus
cd ~
git clone https://github.com/JuaniRaggio/SecurityNexus.git
cd SecurityNexus
```

---

## Deployment del Dashboard

### 1. Compilar Dashboard

```bash
cd ~/SecurityNexus/packages/web-dashboard

# Instalar dependencias
pnpm install

# Compilar para producción
pnpm build

# El output estará en ~/SecurityNexus/packages/web-dashboard/out/
```

### 2. Configurar Variables de Entorno

```bash
# Crear archivo .env.production
cat > ~/SecurityNexus/packages/web-dashboard/.env.production <<EOF
NEXT_PUBLIC_API_URL=https://api.securitynexus.xyz
NEXT_PUBLIC_WS_URL=wss://api.securitynexus.xyz
NODE_ENV=production
EOF
```

### 3. Servir con PM2 (Proceso Manager)

```bash
# Instalar PM2
npm install -g pm2

# Iniciar dashboard con PM2
cd ~/SecurityNexus/packages/web-dashboard
pm2 start npm --name "dashboard" -- start

# Configurar para auto-start
pm2 startup
pm2 save

# Verificar status
pm2 status
pm2 logs dashboard
```

**Alternativa: Servir archivos estáticos con Nginx**

Si usaste `pnpm build` y generaste archivos estáticos:

```bash
# El build está en ~/SecurityNexus/packages/web-dashboard/out/
# Solo necesitas configurar Nginx (ver sección siguiente)
```

---

## Deployment de SAFT Enhanced

### 1. Compilar SAFT Enhanced

```bash
cd ~/SecurityNexus

# Compilar en release mode
cargo build --release --package saft-enhanced

# El binario estará en ~/SecurityNexus/target/release/saft
```

### 2. Crear API Wrapper (Opcional)

Si quieres exponer SAFT como API REST:

```bash
# Crear directorio para el servicio
mkdir -p ~/saft-api
cd ~/saft-api

# Crear servidor simple con Node.js + Express
cat > server.js <<'EOF'
const express = require('express');
const { exec } = require('child_process');
const app = express();
const PORT = 3001;

app.use(express.json());

app.post('/api/analyze', async (req, res) => {
  const { path } = req.body;

  if (!path) {
    return res.status(400).json({ error: 'Path is required' });
  }

  const saftPath = '/home/nexus/SecurityNexus/target/release/saft';
  const cmd = `${saftPath} analyze ${path} --format json`;

  exec(cmd, (error, stdout, stderr) => {
    if (error) {
      return res.status(500).json({ error: stderr });
    }

    try {
      const result = JSON.parse(stdout);
      res.json(result);
    } catch (e) {
      res.status(500).json({ error: 'Failed to parse SAFT output' });
    }
  });
});

app.listen(PORT, () => {
  console.log(`SAFT API running on port ${PORT}`);
});
EOF

# Instalar dependencias
npm init -y
npm install express

# Iniciar con PM2
pm2 start server.js --name "saft-api"
pm2 save
```

---

## Nginx y SSL

### 1. Configurar Nginx para Dashboard

```bash
# Como root
sudo su -

# Crear configuración de Nginx
cat > /etc/nginx/sites-available/security-nexus <<'EOF'
server {
    listen 80;
    server_name securitynexus.xyz www.securitynexus.xyz;

    # Logs
    access_log /var/log/nginx/security-nexus-access.log;
    error_log /var/log/nginx/security-nexus-error.log;

    # Dashboard (Next.js)
    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # SAFT API
    location /api/saft/ {
        proxy_pass http://localhost:3001/api/;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}
EOF

# Activar configuración
ln -s /etc/nginx/sites-available/security-nexus /etc/nginx/sites-enabled/
rm /etc/nginx/sites-enabled/default  # Remover configuración default

# Test y reload
nginx -t
systemctl reload nginx
```

### 2. Configurar SSL con Let's Encrypt

```bash
# Obtener certificado SSL (gratis)
certbot --nginx -d securitynexus.xyz -d www.securitynexus.xyz

# Certbot configurará automáticamente HTTPS
# Renovación automática ya está configurada
```

### 3. Configurar para Archivos Estáticos (Alternativa)

Si usaste static export de Next.js:

```bash
cat > /etc/nginx/sites-available/security-nexus <<'EOF'
server {
    listen 80;
    server_name securitynexus.xyz www.securitynexus.xyz;

    root /home/nexus/SecurityNexus/packages/web-dashboard/out;
    index index.html;

    location / {
        try_files $uri $uri.html $uri/ =404;
    }

    # SAFT API
    location /api/saft/ {
        proxy_pass http://localhost:3001/api/;
    }

    # Cache static assets
    location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }
}
EOF

nginx -t && systemctl reload nginx
```

---

## Monitoring

### 1. Configurar PM2 Monitoring

```bash
# Ver logs en tiempo real
pm2 logs

# Ver status
pm2 status

# Ver métricas
pm2 monit

# Dashboard web (opcional)
pm2 plus
```

### 2. Configurar DigitalOcean Monitoring

```bash
# Vía doctl - habilitar monitoring agent
curl -sSL https://repos.insights.digitalocean.com/install.sh | sudo bash

# El agente enviará métricas a DigitalOcean automáticamente
```

### 3. Configurar Alertas

En el panel de DigitalOcean:
1. Ve a "Monitoring"
2. Configura alertas para:
   - CPU > 80%
   - RAM > 85%
   - Disk > 90%
   - Droplet offline

### 4. Health Checks

```bash
# Crear script de health check
cat > ~/healthcheck.sh <<'EOF'
#!/bin/bash

# Check dashboard
if ! curl -f http://localhost:3000 > /dev/null 2>&1; then
  echo "Dashboard down! Restarting..."
  pm2 restart dashboard
fi

# Check SAFT API
if ! curl -f http://localhost:3001/api/health > /dev/null 2>&1; then
  echo "SAFT API down! Restarting..."
  pm2 restart saft-api
fi
EOF

chmod +x ~/healthcheck.sh

# Agregar a crontab (cada 5 minutos)
crontab -e
# Agregar:
# */5 * * * * /home/nexus/healthcheck.sh >> /home/nexus/healthcheck.log 2>&1
```

---

## Deployment Completo - Script Automatizado

### Script de Deployment Completo

```bash
#!/bin/bash

# deployment.sh - Deployment automatizado para DigitalOcean
# Autor: Security Nexus Team
# Uso: bash deployment.sh

set -e

echo "=== Polkadot Security Nexus - Deployment Script ==="

# Variables
REPO_URL="https://github.com/JuaniRaggio/SecurityNexus.git"
APP_DIR="/home/nexus/SecurityNexus"
DOMAIN="securitynexus.xyz"  # Cambiar por tu dominio

# 1. Actualizar código
echo "📦 Updating repository..."
cd $APP_DIR
git pull origin main

# 2. Compilar SAFT Enhanced
echo "🔨 Building SAFT Enhanced..."
cargo build --release --package saft-enhanced

# 3. Compilar Dashboard
echo "🎨 Building Dashboard..."
cd packages/web-dashboard
pnpm install
pnpm build

# 4. Restart services
echo "🔄 Restarting services..."
pm2 restart dashboard
pm2 restart saft-api

# 5. Verificar health
echo "🏥 Health check..."
sleep 5

if curl -f http://localhost:3000 > /dev/null 2>&1; then
  echo "✅ Dashboard is running"
else
  echo "❌ Dashboard failed to start"
  exit 1
fi

if curl -f http://localhost:3001/api/health > /dev/null 2>&1; then
  echo "✅ SAFT API is running"
else
  echo "⚠️  SAFT API not responding (may be optional)"
fi

echo "🎉 Deployment completed successfully!"
echo "🌐 Dashboard: https://$DOMAIN"
echo "📊 Monitoring: pm2 monit"
```

Guardar como `deployment.sh` y ejecutar:

```bash
chmod +x deployment.sh
./deployment.sh
```

---

## Costos Estimados

### Demo/Pitch (1-2 meses)
```
Droplet $12/mes           = $24 (2 meses)
Dominio $12/año           = $1 (prorrateado)
SSL (Let's Encrypt)       = $0 (gratis)
------------------------------------------
TOTAL:                    = $25 para 2 meses
```

### Producción (6 meses - futuro)
```
Droplet $24/mes           = $144
Managed PostgreSQL $15/mes = $90
Load Balancer $12/mes     = $72
Dominio $12/año           = $6
Spaces $5/mes             = $30
------------------------------------------
TOTAL:                    = $342 para 6 meses
                          = $57/mes promedio
```

---

## Troubleshooting

### Dashboard no carga

```bash
# Verificar logs
pm2 logs dashboard

# Verificar puerto
sudo netstat -tulpn | grep 3000

# Restart
pm2 restart dashboard
```

### Nginx errores

```bash
# Ver logs
tail -f /var/log/nginx/error.log

# Test configuración
nginx -t

# Reload
systemctl reload nginx
```

### SSL no funciona

```bash
# Verificar certificados
certbot certificates

# Renovar manualmente
certbot renew --dry-run
```

### SAFT no compila

```bash
# Verificar Rust version
rustc --version  # Debe ser 1.81 o compatible

# Limpiar y recompilar
cd ~/SecurityNexus
cargo clean
cargo build --release --package saft-enhanced
```

### Out of Memory

```bash
# Agregar swap (temporal)
sudo fallocate -l 2G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile

# Verificar
free -h
```

---

## Checklist de Deployment

### Antes del Deploy
- [ ] Código compilado localmente sin errores
- [ ] Tests pasando
- [ ] Variables de entorno configuradas
- [ ] Dominio apuntando al droplet (si aplica)

### Durante el Deploy
- [ ] SSH key configurada
- [ ] Droplet creado y accesible
- [ ] Firewall configurado
- [ ] Rust y Node.js instalados
- [ ] Código clonado
- [ ] Dashboard compilado
- [ ] SAFT compilado
- [ ] PM2 configurado
- [ ] Nginx configurado
- [ ] SSL configurado

### Después del Deploy
- [ ] Dashboard carga correctamente
- [ ] SSL funcionando (https://)
- [ ] SAFT API responde
- [ ] PM2 auto-restart configurado
- [ ] Monitoring activo
- [ ] Backups configurados (si aplica)

---

## Próximos Pasos

1. **Semana 1**: Deploy básico en DigitalOcean
2. **Semana 2**: Configurar dominio y SSL
3. **Semana 3**: Implementar monitoring y alerts
4. **Semana 4**: Testing y optimización

## Recursos

- [DigitalOcean Docs](https://docs.digitalocean.com/)
- [PM2 Documentation](https://pm2.keymetrics.io/docs/)
- [Nginx Documentation](https://nginx.org/en/docs/)
- [Let's Encrypt](https://letsencrypt.org/)

---

**¿Preguntas o necesitas ayuda?**
- GitHub Issues: https://github.com/JuaniRaggio/SecurityNexus/issues
- Team: Juan Ignacio Raggio & Victoria Helena Park
