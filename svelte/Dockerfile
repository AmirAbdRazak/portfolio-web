FROM node:18 AS build

WORKDIR /app

COPY package.json package-lock.json ./

RUN npm install

COPY . ./

RUN npm run build

FROM node:18-alpine

WORKDIR /app

COPY --from=build /app/build/ ./build

CMD ["node", "build"]