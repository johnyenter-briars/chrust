<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Thanks again! Now go create something AMAZING! :D
***
***
***
*** To avoid retyping too much info. Do a search and replace for the following:
*** johnyenter-briars, chrust, twitter_handle, email, Chrust, project_description
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]



<!-- PROJECT LOGO -->
<br />
<p align="center">
  <a href="https://github.com/johnyenter-briars/chrust">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">Chrust</h3>

  <p align="center">
    project_description
    <br />
    <a href="https://github.com/johnyenter-briars/chrust"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/johnyenter-briars/chrust">View Demo</a>
    ·
    <a href="https://github.com/johnyenter-briars/chrust/issues">Report Bug</a>
    ·
    <a href="https://github.com/johnyenter-briars/chrust/issues">Request Feature</a>
  </p>
</p>



<!-- TABLE OF CONTENTS -->
<details open="open">
  <summary><h2 style="display: inline-block">Table of Contents</h2></summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgements">Acknowledgements</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

[![Product Name Screen Shot][product-screenshot]](https://example.com)

Here's a blank template to get started:
**To avoid retyping too much info. Do a search and replace with your text editor for the following:**
`johnyenter-briars`, `chrust`, `twitter_handle`, `email`, `Chrust`, `project_description`

### Built With

* [Rust](https://www.rust-lang.org/)

<!-- GETTING STARTED -->
## Getting Started

To try your hand at beating rusty, follow the standard installation steps:

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/johnyenter-briars/chrust.git
   ```
2. Build the project
   ```sh
   cargo build
   ```
3. Run the project
   ```sh
   cargo run <args>
   ```


<!-- USAGE EXAMPLES -->
## Usage

The binary currently supports several command line arguments.

```
./chrust [-z TERM|GUI] [-t TICK_SPEED] [-h HUMAN_PLAYS]
```

| Flag | Name | Description |
| ----------- | ----------- | ----------- |
| z | visulization_mode | Sets the visulization mode of the program - either Unicode on the terminal, or graphical TBD |
| t | tick_speed | Sets the interval between moves in milliseconds | 
| h | human_plays | Sets whether or not the human player will play the game. If false, the human player makes random decisions | 

For a complete description on the arguments, run:

```
./chrust --help
```


<!-- ROADMAP -->
## Roadmap

The scope and capabilities of the project will grow and expand as my knowledge of Rust grows and my freetime either inflates or contracts.

Currently, the list of features I would like to add are outlined below.

### AI Features
- [X] MiniMax Implementation
- [ ] Alpha-Beta Pruning
- [ ] Shannon Type B
- [ ] Move Ordering
- [ ] Transposition Tables
- [ ] Quiescence search

### Visual Features
- [ ] Use GUI (or localhost websever)

### 'Program Ease of Use' Features
- [ ] Access to mutable AI difficulty settings
- [ ] Game saves/loads/history


<!-- CONTRIBUTING -->
## Contributing

Feel free to fork, update, change, modify as you see fit. If you would like to explicitly contribute to this project, I would love it if you:

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.



<!-- CONTACT -->
## Contact

John Yenter-Briars - <jyenterbriars@gmail.com>

Project Link: [https://github.com/johnyenter-briars/chrust](https://github.com/johnyenter-briars/chrust)



<!-- ACKNOWLEDGEMENTS -->
## Acknowledgements

* [The Good People of the Rust Discord](https://discord.com/invite/rust)





<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/johnyenter-briars/repo.svg?style=for-the-badge
[contributors-url]: https://github.com/johnyenter-briars/chrust/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/johnyenter-briars/repo.svg?style=for-the-badge
[forks-url]: https://github.com/johnyenter-briars/chrust/network/members
[stars-shield]: https://img.shields.io/github/stars/johnyenter-briars/repo.svg?style=for-the-badge
[stars-url]: https://github.com/johnyenter-briars/chrust/stargazers
[issues-shield]: https://img.shields.io/github/issues/johnyenter-briars/repo.svg?style=for-the-badge
[issues-url]: https://github.com/johnyenter-briars/chrust/issues
[license-shield]: https://img.shields.io/github/license/johnyenter-briars/repo.svg?style=for-the-badge
[license-url]: https://github.com/johnyenter-briars/chrust/blob/master/LICENSE
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/johnyenter-briars
[product-screenshot]: images/screenshot.png