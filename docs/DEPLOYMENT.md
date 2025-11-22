# Docker & Deployment

WebRust is designed to be container-native. It includes a multi-stage `Dockerfile` optimized for production.

## Docker

### Building the Image

The provided `Dockerfile` handles both the frontend (Vite/Tailwind) and backend (Rust) build processes in a single command.

```bash
docker build -t webrust-app -f docker/Dockerfile .
```

### Running the Container

```bash
docker run -p 8000:8000 \
  -e DATABASE_URL="mysql://user:pass@host:3306/dbname" \
  -e APP_KEY="base64:..." \
  webrust-app
```

### Docker Compose

For local development or simple deployments, you can use `docker-compose`.

Create a `docker-compose.yml` file:

```yaml
version: '3.8'
services:
  app:
    build:
      context: .
      dockerfile: docker/Dockerfile
    ports:
      - "8000:8000"
    environment:
      - DATABASE_URL=mysql://webrust:secret@db:3306/webrust
      - APP_ENV=production
    depends_on:
      - db

  db:
    image: mysql:8.0
    environment:
      MYSQL_ROOT_PASSWORD: secret
      MYSQL_DATABASE: webrust
      MYSQL_USER: webrust
      MYSQL_PASSWORD: secret
    volumes:
      - db_data:/var/lib/mysql

volumes:
  db_data:
```

---

## Deployment

Since WebRust compiles to a single binary and static assets, it can be deployed almost anywhere.

### 1. DigitalOcean (App Platform)

DigitalOcean App Platform can build your Dockerfile automatically.

1.  Push your code to GitHub.
2.  Go to DigitalOcean -> Apps -> Create App.
3.  Select your repository.
4.  DigitalOcean will detect the `docker/Dockerfile`.
5.  **Edit Configuration**:
    -   **HTTP Port**: 8000
    -   **Environment Variables**: Add `DATABASE_URL`, `APP_KEY`, etc.
6.  Click **Launch**.

### 2. AWS (App Runner)

AWS App Runner is the easiest way to run containers on AWS.

1.  Push your image to Amazon ECR (Elastic Container Registry).
    ```bash
    aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin <account>.dkr.ecr.us-east-1.amazonaws.com
    docker tag webrust-app <account>.dkr.ecr.us-east-1.amazonaws.com/webrust:latest
    docker push <account>.dkr.ecr.us-east-1.amazonaws.com/webrust:latest
    ```
2.  Go to AWS Console -> App Runner -> Create Service.
3.  Select **Container Registry** and choose your image.
4.  **Configuration**:
    -   **Port**: 8000
    -   **Variables**: Add your env vars here.
5.  Deploy.

### 3. Google Cloud (Cloud Run)

Cloud Run is a serverless container platform.

1.  Build and submit to Google Container Registry (GCR).
    ```bash
    gcloud builds submit --tag gcr.io/PROJECT-ID/webrust
    ```
2.  Deploy to Cloud Run.
    ```bash
    gcloud run deploy webrust \
      --image gcr.io/PROJECT-ID/webrust \
      --platform managed \
      --region us-central1 \
      --allow-unauthenticated \
      --port 8000 \
      --set-env-vars DATABASE_URL="...",APP_KEY="..."
    ```

### 4. Azure (Container Apps)

1.  Push to Azure Container Registry (ACR).
    ```bash
    az acr login --name <registry-name>
    docker tag webrust-app <registry-name>.azurecr.io/webrust:v1
    docker push <registry-name>.azurecr.io/webrust:v1
    ```
2.  Create a Container App.
    ```bash
    az containerapp create \
      --name webrust-app \
      --resource-group my-group \
      --image <registry-name>.azurecr.io/webrust:v1 \
      --target-port 8000 \
      --ingress 'external' \
      --env-vars DATABASE_URL=... APP_KEY=...
    ```

---

## Production Checklist

Before deploying, ensure you have:

1.  **Set `APP_ENV=production`**: This disables debug pages and enables optimizations.
2.  **Generated a Secure Key**: Run `openssl rand -base64 32` and set it as `APP_KEY`.
3.  **Run Migrations**: Ensure your production database is migrated.
    -   You can run migrations as a separate task or part of the startup script (though be careful with multiple instances).
4.  **HTTPS**: Ensure your load balancer or platform provides HTTPS (most of the above do automatically).
