(function() {
    'use strict';
    
    try {
        // Check if localStorage is available
        if (typeof Storage === 'undefined') {
            console.warn('localStorage not available, using default theme');
            return;
        }
        
        const getSystemTheme = () => window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
        const getCurrentTheme = () => localStorage.getItem("theme") || getSystemTheme();
        const theme = getCurrentTheme();
        
        // Validate theme value
        const validThemes = ['light', 'dark'];
        const validatedTheme = validThemes.includes(theme) ? theme : 'light';
        
        const themeStylesheet = document.getElementById("theme-stylesheet");
        if (themeStylesheet) {
            themeStylesheet.setAttribute("href", validatedTheme === "dark" ? "/static/css/dark.css" : "/static/css/light.css");
        }
        
        // Set data attribute for CSS targeting
        document.documentElement.setAttribute('data-theme', validatedTheme);
        
        // Update meta theme-color based on theme
        const metaThemeColor = document.querySelector('meta[name="theme-color"]');
        if (metaThemeColor) {
            metaThemeColor.content = validatedTheme === 'dark' ? '#1a1a1a' : '#2563eb';
        }
        
        if (!localStorage.getItem("theme")) {
            localStorage.setItem("theme", validatedTheme);
        }
    } catch (error) {
        console.error('Error initializing theme:', error);
    }
})();