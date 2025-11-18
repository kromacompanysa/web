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

window.initGridSwiper = () => {
  console.log("🟦 Initializing Grid Swiper…");

  if (typeof Swiper === "undefined") {
    console.warn("⚠️ Swiper not loaded for GRID.");
    return;
  }

  if (window.__gridSwiper) {
    window.__gridSwiper.destroy(true, true);
    window.__gridSwiper = null;
  }

  window.__gridSwiper = new Swiper(".mySwiperBoxes", {
    slidesPerView: 3,
    grid: {
      rows: 2,
      fill: "row",
    },
    spaceBetween: 8,
    pagination: {
      el: ".mySwiperBoxes .swiper-pagination",
      clickable: true,
    },
    breakpoints: {
      320: { slidesPerView: 1, grid: { rows: 2, fill: "row" } },
      640: { slidesPerView: 3, grid: { rows: 2, fill: "row" } },
      1024: { slidesPerView: 5, grid: { rows: 2, fill: "row" } },
    },
  });
};

window.initHeroSwiper = function () {
  console.log("🟦 Initializing Hero Swiper…");

  if (typeof Swiper === "undefined") {
    console.warn("Swiper not loaded");
    return;
  }

  if (window.__heroSwiper) {
    window.__heroSwiper.destroy(true, true);
    window.__heroSwiper = null;
  }

  window.__heroSwiper = new Swiper(".mySwiperHero", {
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
};
