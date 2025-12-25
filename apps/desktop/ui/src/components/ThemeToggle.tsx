import React from "react";
import { useLocalStorage } from "../utils/useLocalStorage";

export function ThemeToggle() {
    const [theme, setTheme] = useLocalStorage<"dark" | "light">("theme", "dark");

    const toggleTheme = () => {
        const newTheme = theme === "dark" ? "light" : "dark";
        setTheme(newTheme);
        document.documentElement.setAttribute("data-theme", newTheme);
    };

    // Set initial theme on mount
    React.useEffect(() => {
        document.documentElement.setAttribute("data-theme", theme);
    }, [theme]);

    return (
        <button className="theme-toggle" onClick={toggleTheme} title="Toggle theme">
            {theme === "dark" ? "☀️" : "🌙"}
        </button>
    );
}
