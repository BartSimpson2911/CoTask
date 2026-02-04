## 🤝 Contributing to cotask

We welcome contributions! Whether it's fixing bugs, improving documentation, or adding new features — your help makes **cotask** better.

---

### 🔁 Contribution Workflow

### 1️⃣ Fork the Repository  
Click the **Fork** button on the top right of the repository page.

---

### 2️⃣ Clone Your Fork

```bash
git clone https://github.com/YOUR-USERNAME/CoTask.git
cd CoTask
```

---

### 3️⃣ Create a New Branch

Never work directly on `main`.

```bash
git checkout -b feature/your-feature-name
```

**Examples**

```bash
feature/add-bisect-command
feature/improve-diff-logic
fix/merge-conflict-bug
```

---

### 4️⃣ Make Your Changes

- Follow Rust best practices  
- Keep functions modular  
- Maintain code readability  

---

### 5️⃣ Test Before Pushing

```bash
cargo build
cargo run -- help
```

Ensure your changes do not break existing functionality.

---

### 6️⃣ Commit Your Work

```bash
git add .
git commit -m "feat: added bisect command"
```

**Commit message types**

| Type | Example |
|------|---------|
| feat | `feat: add interactive rebase` |
| fix | `fix: resolve merge edge case` |
| docs | `docs: update README` |
| refactor | `refactor: simplify commit model` |

---

### 7️⃣ Push to Your Fork

```bash
git push origin feature/your-feature-name
```

---

### 8️⃣ Open a Pull Request

Go to your fork on GitHub and click **“Compare & Pull Request”**.

Your PR should include:

- What you changed  
- Why it’s needed  
- How to test it  

---

### 📌 Contribution Guidelines

✔ Keep PRs focused (one feature per PR)  
✔ Do not commit build artifacts  
✔ Use meaningful commit messages  
✔ Be respectful in discussions  

---

### 🐛 Found a Bug?

Open an issue with:

- Steps to reproduce  
- Expected behavior  
- Actual behavior  
- OS and Rust version  

---

### 💡 Feature Idea?

Please open an issue first to discuss before starting implementation.
