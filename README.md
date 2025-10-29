# 🔐 Criptografia RSA — Mini-Projeto em Rust

**Professor:** Alexandre Montanha 
**Aluno:** Arthur  
**Ferramentas utilizadas:** Notebook LM, RustRover, Copilot

---

## 📚 1. Organização do Material

- A apostila da aula sobre RSA foi adicionada ao Notebook LM.
- O Notebook foi nomeado como: `Criptografia RSA — Segurança da Informação`.

---

## 🎓 2. Mini-Aula: Introdução à Criptografia RSA

### 🔢 Princípios Matemáticos

O algoritmo RSA baseia-se em conceitos da teoria dos números:

- Escolha de dois números primos grandes: `p` e `q`
- Cálculo de `n = p * q` e da função totiente `φ(n) = (p - 1)(q - 1)`
- Escolha de um expoente público `e` tal que `1 < e < φ(n)` e `gcd(e, φ(n)) = 1`
- Cálculo do expoente privado `d`, tal que `d ≡ e⁻¹ mod φ(n)`

### ⚙️ Funcionamento

- **Geração de Chaves:** `(n, e)` é a chave pública; `(n, d)` é a chave privada.
- **Criptografia:** `c = m^e mod n`, onde `m` é a mensagem.
- **Descriptografia:** `m = c^d mod n`, recuperando a mensagem original.

### 🔐 Aplicações Práticas

- Segurança em comunicações digitais (HTTPS, VPNs)
- Assinaturas digitais
- Autenticação de identidade
- Proteção de dados em sistemas bancários e governamentais

---

## 🧪 3. Desenvolvimento Prático em Rust

Um código funcional foi desenvolvido com auxílio do Copilot para implementar o algoritmo RSA. O programa realiza:

- Geração de chaves RSA
- Criptografia de mensagens
- Descriptografia de mensagens
