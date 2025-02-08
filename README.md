<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a id="readme-top"></a>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
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
[![project_license][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <!-- <a href="https://github.com/github_username/repo_name">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a> -->

<h3 align="center">headrs</h3>

  <p align="center">
    A clone of the <b><i>head</i></b> tool using Rust
    <br />
    <a href="https://github.com/sayantandasgupta/headrs"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/sayantandasgupta/headrs">View Demo</a>
    <!-- &middot; -->
    <!-- <a href="https://github.com/github_username/repo_name/issues/new?labels=bug&template=bug-report---.md">Report Bug</a> -->
    <!-- &middot;
    <a href="https://github.com/github_username/repo_name/issues/new?labels=enhancement&template=feature-request---.md">Request Feature</a> -->
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
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
    <!-- <li><a href="#roadmap">Roadmap</a></li> -->
    <!-- <li><a href="#contributing">Contributing</a></li> -->
    <li><a href="#license">License</a></li>
    <!-- <li><a href="#contact">Contact</a></li> -->
    <!-- <li><a href="#acknowledgments">Acknowledgments</a></li> -->
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

<!-- [![Product Name Screen Shot][product-screenshot]](https://example.com) -->

This project is a learning project for learning Rust. This tool is a clone of the `head` tool that is already installed by default in all Unix-based systems. Learning a new language is quite challenging in itself, therefore it is easier to learn if you make projects with the new language. And creating Unix cmd tools is the best way to lay a solid foundation of the language syntax and logic building.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



### Built With

[![Rust][Rust]][Rust-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

This project purely uses Rust, and a Rust crate called `clap` which is a great package to create commandline applications in Rust.

### Prerequisites

To run this project locally, you have to have Rust installed in your system, obviously.

* Rust - for Unix-based OS
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
* Rust - [for Windows](https://forge.rust-lang.org/infra/other-installation-methods.html)

### Installation

1. Fork the repository in your account. After forking a repository should form in your account like
   
   ```sh
   https://github.com/<YOUR_USERNAME>/headrs
   ```
2. Then either use the HTTPS or the SSH URL to clone the repository in your local machine
   
    ```sh
    git clone <URL>
    ```

3. Once the repository has been cloned, change into the source code directory

    ```sh
    cd headrs
    ```

4. Use `cargo` to run and build the project. 

    ```sh
     cargo run -- test.txt
    ```

If you are confused why I am using `cargo`, this project has been built using `cargo` which you can say helps to bundle the source code into a production-ready app. `Cargo` gets installed when you install `Rust`.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage

Here are the different ways this tool may be used

1. Display the first 10 lines of any file
   
   ```
   $> cargo run -- test.txt
      The Project Gutenberg eBook of The Art of War
          
      This ebook is for the use of anyone anywhere in the United States and
      most other parts of the world at no cost and with almost no restrictions
      whatsoever. You may copy it, give it away or re-use it under the terms
      of the Project Gutenberg License included with this ebook or online
      at www.gutenberg.org. If you are not located in the United States,
      you will have to check the laws of the country where you are located
      before using this eBook.
   $>  
   ```

_For more examples, please refer to the [Documentation](https://example.com)_

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->
<!-- ## Roadmap

- [ ] Feature 1
- [ ] Feature 2
- [ ] Feature 3
    - [ ] Nested Feature

See the [open issues](https://github.com/github_username/repo_name/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p> -->



<!-- CONTRIBUTING -->
<!-- ## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Top contributors:

<a href="https://github.com/github_username/repo_name/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=github_username/repo_name" alt="contrib.rocks image" />
</a> -->



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
<!-- ## Contact

Your Name - [@twitter_handle](https://twitter.com/twitter_handle) - email@email_client.com

Project Link: [https://github.com/github_username/repo_name](https://github.com/github_username/repo_name)

<p align="right">(<a href="#readme-top">back to top</a>)</p> -->



<!-- ACKNOWLEDGMENTS -->
<!-- ## Acknowledgments

* []()
* []()
* []()

<p align="right">(<a href="#readme-top">back to top</a>)</p> -->



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/sayantandasgupta/headrs.svg?style=for-the-badge
[contributors-url]: https://github.com/sayantandasgupta/headrs/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/sayantandasgupta/headrs.svg?style=for-the-badge
[forks-url]: https://github.com/sayantandasgupta/headrs/forks
[stars-shield]: https://img.shields.io/github/stars/sayantandasgupta/headrs.svg?style=for-the-badge
[stars-url]: https://github.com/sayantandasgupta/headrs/stargazers
[issues-shield]: https://img.shields.io/github/issues/sayantandasgupta/headrs.svg?style=for-the-badge
[issues-url]: https://github.com/sayantandasgupta/headrs/issues
[license-shield]: https://img.shields.io/github/license/sayantandasgupta/headrs.svg?style=for-the-badge
[license-url]: https://github.com/sayantandasgupta/headrs/blob/master/LICENSE
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/sayantan-dasgupta01
[product-screenshot]: images/screenshot.png
[Rust]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white
[Rust-url]: https://www.rust-lang.org/