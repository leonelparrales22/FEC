# Sistema de Facturación Electrónica FEC

Aplicación de escritorio multiplataforma para facturación electrónica en Ecuador, cumpliendo con los requisitos del SRI.

## Requisitos del Sistema

- Node.js 18+
- Rust 1.70+
- xmlsec1 (para firma XML)
- OpenSSL (para certificados)

### Instalación de Dependencias Nativas

#### Windows
```powershell
choco install openssl
# xmlsec1 via MSYS2 o similar
```

#### macOS
```bash
brew install openssl xmlsec1
```

#### Linux
```bash
sudo apt install libxmlsec1-dev libxmlsec1-openssl openssl
```

## Setup de Desarrollo

1. Clonar el repo:
   ```bash
   git clone https://github.com/leonelparrales22/FEC.git
   cd FEC
   ```

2. Instalar dependencias frontend:
   ```bash
   cd fec-frontend
   npm install
   cd ..
   ```

3. Instalar dependencias Tauri:
   ```bash
   npm install
   ```

4. Construir y ejecutar:
   ```bash
   npm run dev
   ```

## Variables de Entorno

Crear `.env` en la raíz:

```
SRI_ENV=pruebas  # o produccion
SRI_RECEPCION_URL=https://celcer.sri.gob.ec/comprobantes-electronicos-ws/RecepcionComprobantesOffline?wsdl
SRI_AUTORIZACION_URL=https://celcer.sri.gob.ec/comprobantes-electronicos-ws/AutorizacionComprobantesOffline?wsdl
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USER=tuemail@gmail.com
SMTP_PASS=tucontraseña
```

## Generación de Certificados de Prueba

Para pruebas, obtener certificado de prueba del SRI en https://www.sri.gob.ec/facturacion-electronica

## Despliegue

Para producción, cambiar SRI_ENV a produccion y usar certificados válidos.

## Checklist para Producción

- [ ] Registrado como emisor electrónico en SRI
- [ ] Certificado digital válido
- [ ] Configuración de ambiente a producción
- [ ] Pruebas exhaustivas con SRI
- [ ] Backup de base de datos
- [ ] Cumplimiento de retenciones fiscales

## Dependencias

### Frontend
- SvelteKit 2.0
- Tailwind CSS 3.4
- TypeScript 5.0

### Backend
- Tauri 2.0
- Rust 1.70+
- SQLite (rusqlite)
- xmlsec1 para firma XML

Razones: Tauri para app desktop multiplataforma, SvelteKit para UI moderna y responsive, Tailwind para estilos eficientes.

## Plan de Trabajo

1. Configurar proyecto base (completado)
2. Implementar generación de clave de acceso
3. Generación y validación XML
4. Firma XML
5. Integración SRI
6. UI completa
7. PDF y email
8. Base de datos
9. Seguridad
10. Tests y CI/CD
11. Documentación

## Guía de Despliegue

1. Instalar dependencias nativas
2. npm install en raíz y fec-frontend
3. Configurar .env
4. npm run build para paquete
5. Distribuir instaladores generados

## Instrucciones para Pruebas con SRI

1. Usar ambiente pruebas inicialmente
2. Obtener RUC de prueba
3. Configurar certificado de prueba
4. Enviar comprobantes de prueba
5. Verificar estados en portal SRI
6. Una vez validado, cambiar a producción