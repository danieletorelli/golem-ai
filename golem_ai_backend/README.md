
create a new worker

```
golem-cli worker new golem-ai:worker/golem-ai-worker-1 --env OPENAI_API_KEY=
```


ask worker

```
golem-cli worker invoke golem-ai:worker/golem-ai-worker-1  'ask'  '"tell me something about golem cloud"'
```