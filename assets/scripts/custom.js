// assets/scripts/custom.js
// ------------------------------
// 🧩 Global App Initialization
// ------------------------------
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
});

// ------------------------------
// 🌀 Swiper Hero Carousel
// ------------------------------
window.addEventListener("TrunkApplicationStarted", () => {
  if (typeof Swiper === "undefined") {
    return console.warn("⚠️ Swiper not loaded yet for Hero.");
  }

  console.log("🌀 Initializing Hero Swiper...");
  const heroSwiper = new Swiper(".mySwiperHero", {
    slidesPerView: 3,
    spaceBetween: 30,
    pagination: {
      el: ".mySwiperHero .swiper-pagination",
      clickable: true,
    },
    breakpoints: {
      320: { slidesPerView: 1 },
      640: { slidesPerView: 2 },
      1024: { slidesPerView: 3 },
    },
  });
});

// ------------------------------
// 🟦 Swiper Grid Carousel
// ------------------------------
window.addEventListener("TrunkApplicationStarted", () => {
  if (typeof Swiper === "undefined") {
    return console.warn("⚠️ Swiper not loaded yet for Boxes.");
  }

  console.log("🟦 Initializing Grid Swiper...");
  const gridSwiper = new Swiper(".mySwiperBoxes", {
    slidesPerView: 3,
    grid: {
      rows: 2,
      fill: "row",
    },
    spaceBetween: 1,
    pagination: {
      el: ".mySwiperBoxes .swiper-pagination",
      clickable: true,
    },
    breakpoints: {
      320: { slidesPerView: 3, grid: { rows: 2, fill: "row" } },
      640: { slidesPerView: 6, grid: { rows: 2, fill: "row" } },
      1024: { slidesPerView: 8, grid: { rows: 2, fill: "row" } },
    },
  });
});
