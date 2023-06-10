# Example queries

```
curl http://127.0.0.1:8000/health_check

curl -i -X POST -d 'email=thomas_mann@hotmail.com&name=Tom' \
    http://127.0.0.1:8000/subscriptions
```