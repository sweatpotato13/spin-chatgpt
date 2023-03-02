<div id="top"></div>
<p align="center">
<img src=https://img.shields.io/github/stars/sweatpotato13/spin-chatgpt?style=for-the-badge&logo=appveyor&color=blue />
<img src=https://img.shields.io/github/forks/sweatpotato13/spin-chatgpt?style=for-the-badge&logo=appveyor&color=blue />
<img src=https://img.shields.io/github/issues/sweatpotato13/spin-chatgpt?style=for-the-badge&logo=appveyor&color=informational />
<img src=https://img.shields.io/github/issues-pr/sweatpotato13/spin-chatgpt?style=for-the-badge&logo=appveyor&color=informational />
</p>
<br />
<!-- PROJECT LOGO -->
<p align="center">
  <a href="https://www.fermyon.com/" target="blank"><img src="https://i.imgur.com/r3wGmcG.png" width="320" alt="Nest Logo" /></a>
</p>

<br />
<div align="center">
  <a href="https://github.com/sweatpotato13/spin-chatgpt">
    <!-- <img src="images/logo.png" alt="Logo" width="80" height="80"> -->
  </a>

<h3 align="center">spin-chatgpt</h3>

  <p align="center">
    openAI ChatGPT server built with spin
    <br />
    <a href="https://github.com/sweatpotato13/spin-chatgpt"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/sweatpotato13/spin-chatgpt">View Demo</a>
    ·
    <a href="https://github.com/sweatpotato13/spin-chatgpt/issues">Report Bug</a>
    ·
    <a href="https://github.com/sweatpotato13/spin-chatgpt/issues">Request Feature</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
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
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>

### Built With

-   [rust](https://www.rust-lang.org/)
-   [spin](https://developer.fermyon.com/)

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- GETTING STARTED -->

## Getting Started

This is an example of how you may give instructions on setting up your project locally.
To get a local copy up and running follow these simple example steps.

### Installation

1. Clone the repo
    ```sh
    git clone https://github.com/sweatpotato13/spin-chatgpt.git
    cd spin-chatgpt
    ```

### Building

1. Build using cargo:
    ```sh
    spin build
    ```

### Running

0. Write your openAI API key

    you can generate your API key [here](https://platform.openai.com/account/api-keys)

    ```sh
    cp spin.toml.example spin.toml
    nano spin.toml
    ```

    `environment = { OPEN_API_KEY = "sk-FORMATKEY" }`

1. Run using cargo:
    ```sh
    spin up
    ```

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- CONTRIBUTING -->

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- CONTACT -->

## Contact

Project Link: [https://github.com/sweatpotato13/spin-chatgpt](https://github.com/sweatpotato13/spin-chatgpt)
