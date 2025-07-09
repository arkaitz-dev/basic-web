(function() {
    const getSystemTheme = () => window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
    const getCurrentTheme = () => localStorage.getItem("theme") || getSystemTheme();
    const theme = getCurrentTheme();
    const themeStylesheet = document.getElementById("theme-stylesheet");
    if (themeStylesheet) {
        themeStylesheet.setAttribute("href", theme === "dark" ? "/static/css/dark.css" : "/static/css/light.css");
    }
    if (!localStorage.getItem("theme")) localStorage.setItem("theme", theme);
})();