# Babys first rust project

Docker build: 

```docker build -t rusting . ```


Docker run: 

```docker run -p 8000:8000 -it rusting  ```

or both

```docker build -t rusting . &&  docker run -p 8000:8000 -it rusting``` 


Local run

```cargo run```

Local run with watch

```  cargo watch -x "run"  ```