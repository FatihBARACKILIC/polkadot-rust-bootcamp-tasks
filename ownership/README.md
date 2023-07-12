# Ownership

Projeyi kısayol ile çalıştırma için VSCode ile olduğu dizinde açın ve `Command + Shift + B` ye tıklayın.

---

Bu işlemi kendi projenizde de kullanmak isterseniz aşağıdaki adımları takip edebilirsiniz.

1. `Command + Shift + P` ile komut paletini açın.
2. Tasks: Configure Task'ı seçin
3. rust: cargo run'ı seçin
4. Aşağıdaki kodun benzeri oluşacak proje_dizininiz/.vscode/tasks.json dizininde eksik kısımları ekleyin kodunuza.

```JSON
{
    "version": "2.0.0",
    "tasks": [
        {
            "type": "cargo",
            "command": "run",
            "problemMatcher": [
                "$rustc",
                "$rust-panic"
            ],
            "presentation": {
                "echo": true,
                "reveal": "always",
                "focus": false,
                "panel": "shared",
                "showReuseMessage": false,
                "clear": true
            },
            "group": "build",
            "label": "rust: cargo run"
        },
    ]
}
```
