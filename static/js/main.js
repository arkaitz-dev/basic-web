// Theme utilities - available globally
const ThemeUtils = {
  getCurrentTheme: () => {
    return localStorage.getItem("theme") || (window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light");
  },
  
  applyTheme: (theme) => {
    const themeStylesheet = document.getElementById("theme-stylesheet");
    if (themeStylesheet) {
      themeStylesheet.setAttribute("href", theme === "dark" ? "/static/css/dark.css" : "/static/css/light.css");
    }
    
    if (document.body) {
      if (theme === "dark") {
        document.body.classList.add("dark-theme");
        document.body.classList.remove("light-theme");
      } else {
        document.body.classList.add("light-theme");
        document.body.classList.remove("dark-theme");
      }
    }
    
    localStorage.setItem("theme", theme);
  },

  updateThemeIcons: (theme) => {
    const themeToggleButton = document.getElementById("theme-toggle-btn");
    if (!themeToggleButton) return;
    
    const sunIcon = themeToggleButton.querySelector(".sun-icon");
    const moonIcon = themeToggleButton.querySelector(".moon-icon");
    
    if (!sunIcon || !moonIcon) return;
    
    if (theme === "dark") {
      sunIcon.classList.add("icon-hidden");
      moonIcon.classList.remove("icon-hidden");
    } else {
      sunIcon.classList.remove("icon-hidden");
      moonIcon.classList.add("icon-hidden");
    }
  },

  toggleTheme: () => {
    const currentTheme = ThemeUtils.getCurrentTheme();
    const newTheme = currentTheme === "light" ? "dark" : "light";
    ThemeUtils.applyTheme(newTheme);
    ThemeUtils.updateThemeIcons(newTheme);
  }
};

document.addEventListener("DOMContentLoaded", function () {
  var hamburgerBtn = document.getElementById("hamburger-btn");
  var navLinks = document.getElementById("nav-links");

  if (hamburgerBtn && navLinks) {
    hamburgerBtn.addEventListener("click", function (e) {
      e.preventDefault();
      e.stopPropagation();

      var isActive = navLinks.classList.contains("active");

      if (isActive) {
        navLinks.classList.remove("active");
        hamburgerBtn.classList.remove("active");
      } else {
        navLinks.classList.add("active");
        hamburgerBtn.classList.add("active");
      }
    });
    hamburgerBtn.setAttribute('data-listener-attached', 'true'); // Mark as attached
  } else {
    console.error("Hamburger button or nav links not found.");
  }

  // Theme Switcher Logic
  const themeToggleButton = document.getElementById("theme-toggle-btn");
  
  if (themeToggleButton) {
    // Initialize theme and icons to match current theme
    const currentTheme = ThemeUtils.getCurrentTheme();
    ThemeUtils.applyTheme(currentTheme);
    ThemeUtils.updateThemeIcons(currentTheme);

    // Theme toggle button click handler
    themeToggleButton.addEventListener("click", (e) => {
      e.preventDefault();
      e.stopPropagation();
      ThemeUtils.toggleTheme();
    });
  } else {
    console.error("Theme toggle button not found!");
  }

  // Close menu when a nav link is clicked
  if (navLinks) {
    navLinks.addEventListener("click", function (e) {
      if (e.target.tagName === "A" || e.target.closest("A")) {
        // Check if the click is on an anchor or its child
        if (navLinks.classList.contains("active")) {
          navLinks.classList.remove("active");
          if (hamburgerBtn) {
            hamburgerBtn.classList.remove("active");
          }
        }
      }
    });
  }

  // Close menu with Escape key
  document.addEventListener("keydown", function (e) {
    if (e.key === "Escape") {
      if (navLinks && navLinks.classList.contains("active")) {
        navLinks.classList.remove("active");
        if (hamburgerBtn) {
          hamburgerBtn.classList.remove("active");
        }
      }
    }
  });
});

// Ensure htmx processing happens after our DOMContentLoaded setup
// This might be useful if htmx swaps content that includes the menu
document.body.addEventListener("htmx:afterSwap", function (event) {
  // Potentially re-initialize or re-check elements if they are part of swapped content.
  // For a static header, this might not be strictly necessary unless the header itself is swapped.
  // For now, we'll assume the header is static and the initial setup is sufficient.
  // If issues arise where the menu stops working after htmx swaps,
  // you might need to re-attach event listeners here or ensure IDs remain unique.

  var hamburgerBtn = document.getElementById("hamburger-btn");
  var navLinks = document.getElementById("nav-links");

  if (
    hamburgerBtn &&
    navLinks &&
    !hamburgerBtn.getAttribute("data-listener-attached")
  ) {
    hamburgerBtn.addEventListener("click", function (e) {
      e.preventDefault();
      e.stopPropagation();
      var isActive = navLinks.classList.contains("active");
      if (isActive) {
        navLinks.classList.remove("active");
        hamburgerBtn.classList.remove("active");
      } else {
        navLinks.classList.add("active");
        hamburgerBtn.classList.add("active");
      }
    });
    hamburgerBtn.setAttribute("data-listener-attached", "true"); // Mark as attached
  }

  // Update active nav link highlighting
  if (navLinks) { // navLinks is the <ul> element with id="nav-links", fetched earlier in this event listener
    const allAnchorElements = navLinks.querySelectorAll('a');
    const currentPath = window.location.pathname;

    allAnchorElements.forEach(anchor => {
      anchor.classList.remove('active');
      // Compare the anchor's href attribute (e.g., "/about") with the current path
      if (anchor.getAttribute('href') === currentPath) {
        anchor.classList.add('active');
      }
    });
  }

  // Ensure nav links also close the menu after htmx swaps, if the nav-links container itself could be swapped.
  // This is less likely for a global nav.
});
