// src/lib/state/theme.svelte.ts
import { browser } from '$app/environment';

class ThemeState {
    isDark = $state(true);

    constructor() {
        if (browser) {
            const stored = localStorage.getItem('theme');
            if (stored) {
                this.isDark = stored === 'dark';
            } else {
                this.isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
            }
            this.apply();
        }
    }

    toggle() {
        this.isDark = !this.isDark;
        this.apply();
    }

    private apply() {
        if (!browser) return;
        const root = document.documentElement;
        if (this.isDark) {
            root.classList.add('dark');
            localStorage.setItem('theme', 'dark');
        } else {
            root.classList.remove('dark');
            localStorage.setItem('theme', 'light');
        }
    }
}

// Create a singleton
export const themeStore = new ThemeState();
