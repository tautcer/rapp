To run it:
In one terminal run this
```sh
yarn tailwindcss -i styles/tailwind.css -o assets/main.css --watch
```

In the second run this:

```sh
watchexec -r -e css,html,rs cargo run
```
