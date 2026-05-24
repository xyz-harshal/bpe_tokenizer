**Performance:**
- `rev_vocab` rebuilt every `decode` call — move it to struct or build once
- `words_split` in encode stores word string unnecessarily — only `Vec<String>` needed

**Code quality:**
- `vocab_size` hardcoded inside `train` — should be a parameter `train(&mut self, input: &str, vocab_size: usize)`
- Magic number `k = 1` — why not `0`?

---
