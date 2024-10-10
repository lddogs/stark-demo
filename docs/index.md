---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "STARK demo"
  tagline: STARK - Redefining trust.
  actions:
    - theme: brand
      text: Docs
      link: /markdown-examples
  image:
    src: /logo-large.jpg
    alt: VitePress
features:
  - title: Transparency
    details: STARK uses algorithms that are generally easier to understand and more transparent compared to other zk-SNARKs, fostering greater trust in the system.
  - title: Scalability
    details: STARK produces proofs that are significantly smaller in size, reducing storage and transmission costs. Additionally, the verification process for STARK proofs is typically much faster, making it suitable for high-performance applications.
  - title: Flexibility
    details: STARK can be used to prove a wide range of computations, including complex ones. Moreover, it offers a high degree of customization to fit specific application requirements.
  - title: Privacy
    details: Similar to other zk-SNARKs, STARK allows a prover to demonstrate the correctness of a statement without revealing any information beyond the statement's validity.
  - title: Trustworthiness
    details: STARK has a lower reliance on trusted setup parameters, reducing the risks associated with these parameters being compromised.
  - title: Quantum resistance
    details: One of STARK's most notable advantages is its resistance to attacks from quantum computers. This makes STARK a more future-proof security solution compared to other technologies.
---

<style>
:root {
  --vp-home-hero-name-color: transparent;
  --vp-home-hero-name-background: -webkit-linear-gradient(120deg, #53B6CC 30%, #E16413);

  --vp-home-hero-image-background-image: linear-gradient(-45deg, #53B6CC 50%, #E16413 50%);
  --vp-home-hero-image-filter: blur(44px);
}

@media (min-width: 640px) {
  :root {
    --vp-home-hero-image-filter: blur(56px);
  }
}

@media (min-width: 960px) {
  :root {
    --vp-home-hero-image-filter: blur(68px);
  }
}
</style>