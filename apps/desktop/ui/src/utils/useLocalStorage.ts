import { useState, useEffect } from "react";

type Updater<T> = T | ((prev: T) => T);

export function useLocalStorage<T>(key: string, initialValue: T): [T, (value: Updater<T>) => void] {
    const [storedValue, setStoredValue] = useState<T>(() => {
        try {
            const item = window.localStorage.getItem(key);
            return item ? JSON.parse(item) : initialValue;
        } catch {
            return initialValue;
        }
    });

    const setValue = (value: Updater<T>) => {
        try {
            const nextValue = value instanceof Function ? value(storedValue) : value;
            setStoredValue(nextValue);
            window.localStorage.setItem(key, JSON.stringify(nextValue));
        } catch (error) {
            console.error("Error saving to localStorage:", error);
        }
    };

    return [storedValue, setValue];
}
