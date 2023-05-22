FROM rust:latest as rust-builder

# Instalar Rust nightly
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
ENV PATH="/root/.cargo/bin:${PATH}"

# Establecer el directorio de trabajo para el backend
WORKDIR /app/backend

# Copiar los archivos de la API en Rust
COPY ./backend .

# Compilar la API en Rust
RUN cargo build --release

# Copiar la base de datos SQLite a la carpeta de salida de la compilación
COPY ./backend/housings.sqlite ./target/release/

# Imagen base para Node.js
FROM node:latest as node-builder

# Establecer el directorio de trabajo para el frontend
WORKDIR /app/frontend

# Copiar los archivos del frontend en Angular
COPY ./frontend .

# Instalar dependencias del frontend
RUN npm install

# Construir la aplicación frontend
RUN npm run build

# Imagen final para el contenedor
FROM rust:latest

# Establecer el directorio de trabajo
WORKDIR /app

# Copiar los archivos compilados del backend
COPY --from=rust-builder /app/backend/target/release/housing-api .
COPY --from=rust-builder /app/backend/target/release/housings.sqlite .

# Copiar los archivos compilados del frontend
COPY --from=node-builder /app/frontend/dist /app/frontend

# Establecer la variable de entorno DATABASE_URL
ENV DATABASE_URL="sqlite://housings.sqlite"

# Actualizar y instalar npm
RUN apt-get update && apt-get install -y npm

# Instalar npx
RUN npm install -g npx

# Instalar http-server
RUN npm install -g http-server@0.12.3

# Exponer el puerto de la API
EXPOSE 8000

# Comando para ejecutar el backend y el frontend
CMD ["sh", "-c", "./housing-api & cd frontend && npx http-server"]
