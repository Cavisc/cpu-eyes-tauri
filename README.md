<p align="center">
    <img width="256" height="256" src="./assets/cpu_eyes.gif"/>
</p>

# CPU Eyes

Um monitor de CPU simples e eficiente, construído com Tauri, Vite e React. O CPU Eyes fornece informações detalhadas sobre o seu sistema, incluindo:

- **Uso da CPU:** A porcentagem atual de utilização da CPU.
- **Temperatura da CPU:** A temperatura atual do processador, dependendo do sistema operacional.
- **Velocidade do clock:** A frequência média do processador, em GHz.
- **Uso da RAM:** A quantidade de memória RAM em uso, em GB.

## Tecnologias Utilizadas

- **[Tauri](https://tauri.app/):** Framework para criar aplicativos de desktop com segurança e eficiência, utilizando Rust e tecnologias web.
- **[Vite](https://vitejs.dev/):** Ferramenta de build rápida e moderna para projetos web.
- **[React](https://reactjs.org/):** Biblioteca JavaScript para criar interfaces de usuário.
- **[sysinfo](https://crates.io/crates/sysinfo):** Biblioteca em Rust para acessar informações detalhadas sobre o sistema.

## Build

Para construir o aplicativo para diferentes sistemas operacionais, utilize os seguintes comandos:

- **Windows:** `npx tauri build`
- **macOS:** `npx tauri build`
- **Linux:** `npx tauri build`

**OBS:** A aplicação foi desenvolvida e testada em múltiplos sistemas operacionais, mas algumas funcionalidades podem variar dependendo do ambiente.
