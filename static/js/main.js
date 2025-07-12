/* ===============================================
   Professional Portfolio JavaScript
   Enhanced accessibility and UX features
   =============================================== */

// Advanced Theme Management with Accessibility Features
const ThemeManager = {
  // Theme preference detection with system preference fallback
  getCurrentTheme() {
    const stored = localStorage.getItem("theme");
    if (stored && (stored === "light" || stored === "dark")) {
      return stored;
    }
    
    // Respect system preference
    return window.matchMedia?.("(prefers-color-scheme: dark)").matches 
      ? "dark" 
      : "light";
  },

  // Apply theme with smooth transitions
  applyTheme(theme) {
    const themeStylesheet = document.getElementById("theme-stylesheet");
    if (!themeStylesheet) return;

    // Prevent flash of unstyled content
    document.documentElement.style.setProperty('--transition-theme', 'none');
    
    // Update stylesheet
    const newHref = theme === "dark" 
      ? "/static/css/dark.css" 
      : "/static/css/light.css";
    
    themeStylesheet.setAttribute("href", newHref);
    
    // Update body classes for additional styling hooks
    document.body.className = document.body.className
      .replace(/\b(light|dark)-theme\b/g, '')
      .trim();
    document.body.classList.add(`${theme}-theme`);
    
    // Store preference
    localStorage.setItem("theme", theme);
    
    // Re-enable transitions after a brief delay
    requestAnimationFrame(() => {
      document.documentElement.style.removeProperty('--transition-theme');
    });

    // Announce theme change to screen readers
    this.announceThemeChange(theme);
  },

  // Update theme toggle button with proper ARIA attributes
  updateThemeButton(theme) {
    const button = document.getElementById("theme-toggle");
    if (!button) return;

    const sunIcon = button.querySelector(".theme-icon-sun");
    const moonIcon = button.querySelector(".theme-icon-moon");
    
    if (sunIcon && moonIcon) {
      if (theme === "dark") {
        sunIcon.style.display = "none";
        moonIcon.style.display = "block";
        button.setAttribute("aria-label", "Switch to light mode");
        button.setAttribute("title", "Switch to light mode");
      } else {
        sunIcon.style.display = "block";
        moonIcon.style.display = "none";
        button.setAttribute("aria-label", "Switch to dark mode");
        button.setAttribute("title", "Switch to dark mode");
      }
    }
  },

  // Announce theme changes to screen readers
  announceThemeChange(theme) {
    const announcement = document.createElement("div");
    announcement.setAttribute("aria-live", "polite");
    announcement.setAttribute("aria-atomic", "true");
    announcement.className = "visually-hidden";
    announcement.textContent = `Theme changed to ${theme} mode`;
    
    document.body.appendChild(announcement);
    
    // Remove after announcement
    setTimeout(() => {
      document.body.removeChild(announcement);
    }, 1000);
  },

  // Toggle theme with keyboard support
  toggle() {
    const current = this.getCurrentTheme();
    const newTheme = current === "light" ? "dark" : "light";
    
    this.applyTheme(newTheme);
    this.updateThemeButton(newTheme);
  },

  // Initialize theme system
  init() {
    const theme = this.getCurrentTheme();
    this.applyTheme(theme);
    this.updateThemeButton(theme);

    // Listen for system theme changes
    if (window.matchMedia) {
      const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
      mediaQuery.addEventListener("change", (e) => {
        // Only auto-switch if user hasn't set a preference
        if (!localStorage.getItem("theme")) {
          const systemTheme = e.matches ? "dark" : "light";
          this.applyTheme(systemTheme);
          this.updateThemeButton(systemTheme);
        }
      });
    }
  }
};

// Enhanced Navigation with Accessibility
const NavigationManager = {
  init() {
    this.setupMobileMenu();
    this.setupKeyboardNavigation();
    this.setupActiveStates();
    this.setupSmoothScrolling();
  },

  setupMobileMenu() {
    const toggleButton = document.getElementById("mobile-menu-toggle");
    const navLinks = document.getElementById("nav-links");
    
    if (!toggleButton || !navLinks) {
      console.log("Mobile menu elements not found:", {
        toggleButton: !!toggleButton,
        navLinks: !!navLinks
      });
      return;
    }

    // Enhanced click handler with proper ARIA states
    toggleButton.addEventListener("click", (e) => {
      e.preventDefault();
      e.stopPropagation();
      
      const isExpanded = toggleButton.getAttribute("aria-expanded") === "true";
      const newState = !isExpanded;
      
      // Update ARIA states
      toggleButton.setAttribute("aria-expanded", newState);
      navLinks.setAttribute("aria-hidden", !newState);
      
      // Update visual states
      toggleButton.classList.toggle("active", newState);
      navLinks.classList.toggle("active", newState);
      
      // Manage focus for accessibility
      if (newState) {
        navLinks.querySelector("a")?.focus();
      }
    });

    // Close menu when clicking outside
    document.addEventListener("click", (e) => {
      if (!toggleButton.contains(e.target) && !navLinks.contains(e.target)) {
        this.closeMobileMenu();
      }
    });

    // Close menu on nav link click
    navLinks.addEventListener("click", (e) => {
      if (e.target.matches("a")) {
        this.closeMobileMenu();
      }
    });
  },

  closeMobileMenu() {
    const toggleButton = document.getElementById("mobile-menu-toggle");
    const navLinks = document.getElementById("nav-links");
    
    if (toggleButton && navLinks) {
      toggleButton.setAttribute("aria-expanded", "false");
      navLinks.setAttribute("aria-hidden", "true");
      toggleButton.classList.remove("active");
      navLinks.classList.remove("active");
    }
  },

  setupKeyboardNavigation() {
    // Enhanced keyboard navigation
    document.addEventListener("keydown", (e) => {
      // Close mobile menu with Escape
      if (e.key === "Escape") {
        this.closeMobileMenu();
        
        // Return focus to toggle button
        const toggleButton = document.getElementById("mobile-menu-toggle");
        if (toggleButton && toggleButton.getAttribute("aria-expanded") === "false") {
          toggleButton.focus();
        }
      }
      
      // Theme toggle with keyboard shortcut (Ctrl/Cmd + Shift + T)
      if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === "T") {
        e.preventDefault();
        ThemeManager.toggle();
      }
    });
  },

  setupActiveStates() {
    this.updateActiveNavLink();
    
    // Update active state after htmx navigation
    document.body.addEventListener("htmx:afterSwap", () => {
      this.updateActiveNavLink();
    });
  },

  updateActiveNavLink() {
    const navLinks = document.querySelectorAll("#nav-links a");
    const currentPath = window.location.pathname;
    
    navLinks.forEach(link => {
      link.classList.remove("active");
      link.removeAttribute("aria-current");
      
      if (link.getAttribute("href") === currentPath) {
        link.classList.add("active");
        link.setAttribute("aria-current", "page");
      }
    });
  },

  setupSmoothScrolling() {
    // Enhanced smooth scrolling for anchor links
    document.addEventListener("click", (e) => {
      const link = e.target.closest("a[href^='#']");
      if (!link) return;
      
      const targetId = link.getAttribute("href").substring(1);
      const targetElement = document.getElementById(targetId);
      
      if (targetElement) {
        e.preventDefault();
        
        // Respect user motion preferences
        const shouldReduceMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
        
        targetElement.scrollIntoView({
          behavior: shouldReduceMotion ? "auto" : "smooth",
          block: "start"
        });
        
        // Update focus for accessibility
        targetElement.setAttribute("tabindex", "-1");
        targetElement.focus();
        
        // Remove tabindex after focus
        targetElement.addEventListener("blur", () => {
          targetElement.removeAttribute("tabindex");
        }, { once: true });
      }
    });
  }
};

// Accessibility Enhancements
const AccessibilityManager = {
  init() {
    this.setupColorblindSupport();
    this.setupReducedMotion();
    this.setupFocusManagement();
    this.setupARIALiveRegions();
  },

  setupColorblindSupport() {
    // Add visual patterns to complement color information
    this.enhanceFormValidation();
    this.enhanceStatusMessages();
  },

  enhanceFormValidation() {
    // Add icons and patterns to form validation states
    const forms = document.querySelectorAll("form");
    
    forms.forEach(form => {
      form.addEventListener("submit", (e) => {
        const inputs = form.querySelectorAll("input, textarea");
        
        inputs.forEach(input => {
          if (!input.validity.valid) {
            // Add error icon for colorblind users
            this.addValidationIcon(input, "error");
          } else {
            this.addValidationIcon(input, "success");
          }
        });
      });
    });
  },

  addValidationIcon(input, type) {
    // Remove existing icons
    const existingIcon = input.parentNode.querySelector(".validation-icon");
    if (existingIcon) {
      existingIcon.remove();
    }
    
    // Add new icon
    const icon = document.createElement("span");
    icon.className = `validation-icon validation-icon-${type}`;
    icon.setAttribute("aria-hidden", "true");
    
    if (type === "error") {
      icon.innerHTML = "⚠"; // Warning symbol
      icon.setAttribute("title", "Error");
    } else {
      icon.innerHTML = "✓"; // Check mark
      icon.setAttribute("title", "Valid");
    }
    
    input.parentNode.appendChild(icon);
  },

  enhanceStatusMessages() {
    // Add patterns to alert messages
    const alerts = document.querySelectorAll(".alert");
    
    alerts.forEach(alert => {
      if (!alert.querySelector(".alert-icon")) {
        const icon = document.createElement("span");
        icon.className = "alert-icon";
        icon.setAttribute("aria-hidden", "true");
        
        if (alert.classList.contains("alert-success")) {
          icon.innerHTML = "✓";
        } else if (alert.classList.contains("alert-error")) {
          icon.innerHTML = "⚠";
        } else if (alert.classList.contains("alert-warning")) {
          icon.innerHTML = "⚠";
        } else if (alert.classList.contains("alert-info")) {
          icon.innerHTML = "ℹ";
        }
        
        alert.insertBefore(icon, alert.firstChild);
      }
    });
  },

  setupReducedMotion() {
    // Respect user motion preferences
    if (window.matchMedia("(prefers-reduced-motion: reduce)").matches) {
      document.documentElement.style.setProperty("--transition-fast", "0ms");
      document.documentElement.style.setProperty("--transition-base", "0ms");
      document.documentElement.style.setProperty("--transition-slow", "0ms");
    }
  },

  setupFocusManagement() {
    // Enhanced focus management for better keyboard navigation
    let focusableElements = [];
    
    // Update focusable elements list
    const updateFocusableElements = () => {
      focusableElements = Array.from(document.querySelectorAll(
        'a[href], button:not([disabled]), input:not([disabled]), textarea:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex="-1"])'
      ));
    };
    
    updateFocusableElements();
    
    // Re-update after DOM changes
    document.body.addEventListener("htmx:afterSwap", updateFocusableElements);
    
    // Trap focus in mobile menu when open
    document.addEventListener("keydown", (e) => {
      if (e.key === "Tab") {
        const mobileMenu = document.getElementById("nav-links");
        if (mobileMenu && mobileMenu.classList.contains("active")) {
          const menuFocusableElements = Array.from(mobileMenu.querySelectorAll("a"));
          const firstElement = menuFocusableElements[0];
          const lastElement = menuFocusableElements[menuFocusableElements.length - 1];
          
          if (e.shiftKey && document.activeElement === firstElement) {
            e.preventDefault();
            lastElement.focus();
          } else if (!e.shiftKey && document.activeElement === lastElement) {
            e.preventDefault();
            firstElement.focus();
          }
        }
      }
    });
  },

  setupARIALiveRegions() {
    // Create ARIA live region for dynamic announcements
    if (!document.getElementById("aria-live-region")) {
      const liveRegion = document.createElement("div");
      liveRegion.id = "aria-live-region";
      liveRegion.setAttribute("aria-live", "polite");
      liveRegion.setAttribute("aria-atomic", "true");
      liveRegion.className = "visually-hidden";
      document.body.appendChild(liveRegion);
    }
  },

  announce(message) {
    const liveRegion = document.getElementById("aria-live-region");
    if (liveRegion) {
      liveRegion.textContent = message;
      
      // Clear after announcement
      setTimeout(() => {
        liveRegion.textContent = "";
      }, 1000);
    }
  }
};

// Performance and Loading Manager
const LoadingManager = {
  init() {
    this.setupLazyLoading();
    this.setupProgressiveEnhancement();
  },

  setupLazyLoading() {
    // Lazy load images with intersection observer
    if ("IntersectionObserver" in window) {
      const imageObserver = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
          if (entry.isIntersecting) {
            const img = entry.target;
            if (img.dataset.src) {
              img.src = img.dataset.src;
              img.removeAttribute("data-src");
              imageObserver.unobserve(img);
            }
          }
        });
      });

      document.querySelectorAll("img[data-src]").forEach(img => {
        imageObserver.observe(img);
      });
    }
  },

  setupProgressiveEnhancement() {
    // Add 'js-enabled' class for progressive enhancement
    document.documentElement.classList.add("js-enabled");
    
    // Remove no-js fallbacks
    document.querySelectorAll(".no-js-only").forEach(el => {
      el.style.display = "none";
    });
  }
};

// Initialize everything when DOM is ready
document.addEventListener("DOMContentLoaded", () => {
  ThemeManager.init();
  NavigationManager.init();
  AccessibilityManager.init();
  LoadingManager.init();
  
  // Setup theme toggle button
  const themeToggle = document.getElementById("theme-toggle");
  if (themeToggle) {
    themeToggle.addEventListener("click", (e) => {
      e.preventDefault();
      ThemeManager.toggle();
    });
  }
});

// Handle HTMX events for SPA-like behavior
document.body.addEventListener("htmx:beforeSwap", (e) => {
  // Add loading class for smooth transitions
  document.body.classList.add("htmx-loading");
});

document.body.addEventListener("htmx:afterSwap", (e) => {
  // Remove loading class
  document.body.classList.remove("htmx-loading");
  
  // Re-initialize dynamic elements
  AccessibilityManager.enhanceStatusMessages();
  
  // Announce navigation to screen readers
  const pageTitle = document.querySelector("h1")?.textContent || "Page loaded";
  AccessibilityManager.announce(`Navigated to ${pageTitle}`);
});

// Export for global access
window.ThemeManager = ThemeManager;
window.AccessibilityManager = AccessibilityManager;