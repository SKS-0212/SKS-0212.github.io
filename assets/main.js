
/* =========================================================
   Rustfolio — Main JavaScript
   Theme Toggle + Cursor Glow
   ========================================================= */

(() => {
    "use strict";

    const root = document.documentElement;
    const THEME_KEY = "rustfolio-theme";

    const prefersDark = window.matchMedia(
        "(prefers-color-scheme: dark)"
    );

    /* =====================================================
       SVG Icons
       ===================================================== */

    const moonIcon = `
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="20"
            height="20"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="1.8"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
        >
            <path
                d="M20.985 12.486A9 9 0 0 1
                   11.514 3.015a9 9 0 1 0
                   9.47 9.47z"
            />
        </svg>
    `;

    const sunIcon = `
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="20"
            height="20"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="1.8"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
        >
            <circle cx="12" cy="12" r="4" />
            <path d="M12 2v2" />
            <path d="M12 20v2" />
            <path d="M4.93 4.93l1.42 1.42" />
            <path d="M17.65 17.65l1.42 1.42" />
            <path d="M2 12h2" />
            <path d="M20 12h2" />
            <path d="M6.35 17.65l-1.42 1.42" />
            <path d="M19.07 4.93l-1.42 1.42" />
        </svg>
    `;

    /* =====================================================
       Theme Storage
       ===================================================== */

    function getSavedTheme() {
        try {
            const theme = localStorage.getItem(THEME_KEY);

            if (theme === "light" || theme === "dark") {
                return theme;
            }
        } catch (_) {
            // Local storage may be unavailable.
        }

        return null;
    }

    function saveTheme(theme) {
        try {
            localStorage.setItem(THEME_KEY, theme);
        } catch (_) {
            // Theme still works for the current page.
        }
    }

    /* =====================================================
       Theme Management
       ===================================================== */

    function getPreferredTheme() {
        return prefersDark.matches ? "dark" : "light";
    }

    function updateThemeButtons(theme) {
        const isDark = theme === "dark";

        document
            .querySelectorAll("[data-theme-toggle]")
            .forEach((button) => {
                button.innerHTML = isDark
                    ? sunIcon
                    : moonIcon;

                button.setAttribute(
                    "aria-label",
                    isDark
                        ? "Switch to light mode"
                        : "Switch to dark mode"
                );

                button.setAttribute(
                    "aria-pressed",
                    String(isDark)
                );

                button.title = isDark
                    ? "Switch to light mode"
                    : "Switch to dark mode";
            });
    }

    function applyTheme(theme, persist = false) {
        root.dataset.theme = theme;

        updateThemeButtons(theme);

        if (persist) {
            saveTheme(theme);
        }
    }

    function toggleTheme() {
        const currentTheme =
            root.dataset.theme === "dark"
                ? "dark"
                : "light";

        const nextTheme =
            currentTheme === "dark"
                ? "light"
                : "dark";

        applyTheme(nextTheme, true);
    }

    /* =====================================================
       Initialize Theme
       ===================================================== */

    function initializeTheme() {
        const savedTheme = getSavedTheme();

        applyTheme(
            savedTheme || getPreferredTheme()
        );

        document
            .querySelectorAll("[data-theme-toggle]")
            .forEach((button) => {
                button.addEventListener(
                    "click",
                    toggleTheme
                );
            });
    }

    /* =====================================================
       System Theme Changes
       ===================================================== */

    function handleSystemThemeChange(event) {
        if (getSavedTheme() === null) {
            applyTheme(
                event.matches ? "dark" : "light"
            );
        }
    }

    if (typeof prefersDark.addEventListener === "function") {
        prefersDark.addEventListener(
            "change",
            handleSystemThemeChange
        );
    } else if (typeof prefersDark.addListener === "function") {
        prefersDark.addListener(
            handleSystemThemeChange
        );
    }

    /* =====================================================
       Cursor Glow
       ===================================================== */

    function initializeCursorGlow() {
        const supportsHover = window.matchMedia(
            "(hover: hover) and (pointer: fine)"
        ).matches;

        const reducedMotion = window.matchMedia(
            "(prefers-reduced-motion: reduce)"
        ).matches;

        if (!supportsHover || reducedMotion) {
            return;
        }

        let mouseX = window.innerWidth / 2;
        let mouseY = window.innerHeight / 2;
        let framePending = false;

        document.addEventListener(
            "pointermove",
            (event) => {
                mouseX = event.clientX;
                mouseY = event.clientY;

                if (framePending) {
                    return;
                }

                framePending = true;

                requestAnimationFrame(() => {
                    root.style.setProperty(
                        "--mouse-x",
                        `${mouseX}px`
                    );

                    root.style.setProperty(
                        "--mouse-y",
                        `${mouseY}px`
                    );

                    framePending = false;
                });
            },
            { passive: true }
        );
    }

    /* =====================================================
       Initialize
       ===================================================== */

    function initialize() {
        initializeTheme();
        initializeCursorGlow();
    }

    if (document.readyState === "loading") {
        document.addEventListener(
            "DOMContentLoaded",
            initialize,
            { once: true }
        );
    } else {
        initialize();
    }
})();
