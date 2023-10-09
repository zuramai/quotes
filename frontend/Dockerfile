FROM node:alpine

WORKDIR /frontend 

COPY . .

RUN npm install -g pnpm
RUN pnpm install 

CMD ["pnpm", "dev"]