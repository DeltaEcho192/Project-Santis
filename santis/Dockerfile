FROM node:current-slim

WORKDIR /usr/src/app
COPY package.json .
RUN npm install
COPY . .

EXPOSE 2502 
CMD ["node", "server.js"]
