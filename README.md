# dagensquiz.no / dailyquiz.app

Podman:

```bash
podman build --tag dagensquiz . && podman run -it -p 3000:3000 --replace --name dagensquiz --rm dagensquiz
```

Docker:

```bash
docker build --tag dagensquiz . && docker run -it -p 3000:3000 --name dagensquiz --rm dagensquiz
```
