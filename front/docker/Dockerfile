
FROM node:alpine as build

WORKDIR /app

COPY ./package*.json ./

RUN npm install 

COPY . .

RUN npm run build

FROM nginx:alpine

COPY docker/nginx.conf.template /etc/nginx/templates/default.conf.template

COPY --from=build /app/dist /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]