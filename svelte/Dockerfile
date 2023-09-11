# syntax = docker/dockerfile:1

# Adjust NODE_VERSION as desired
FROM oven/bun

WORKDIR /app

COPY package.json package.json
RUN bun install

COPY . .

RUN bun run build

EXPOSE 3000
CMD ["bun", "./build"]
