// assets/scripts/custom.js
window.addEventListener("TrunkApplicationStarted", () => {
  console.log("✅ Leptos app mounted — initializing UI scripts");

  // --- Hamburger menu toggle ---
  const menuBtn = document.getElementById("menu-btn");
  const mobileMenu = document.getElementById("mobile-menu");
  if (menuBtn && mobileMenu) {
    menuBtn.addEventListener("click", () => {
      mobileMenu.classList.toggle("hidden");
      console.log("🍔 Menu toggled");
    });
  } else {
    console.warn("Menu elements not found yet.");
  }

  // --- Swiper Carousel ---
  if (typeof Swiper !== "undefined") {
    console.log("🌀 Initializing Swiper...");
    new Swiper(".mySwiper", {
      slidesPerView: 3,
      spaceBetween: 30,
      pagination: {
        el: ".swiper-pagination",
        clickable: true,
      },
      breakpoints: {
        320: { slidesPerView: 1 },
        640: { slidesPerView: 2 },
        1024: { slidesPerView: 3 },
      },
    });
  } else {
    console.warn("Swiper not loaded yet.");
  }
});
