// https://stackoverflow.com/questions/68424114/next-js-how-to-fetch-localstorage-data-before-client-side-rendering
// 解决 nextJS 无法获取初始localstorage问题

import { useEffect, useState } from 'react';

const isSSR = true

const getDefaultStorage = (key: string) => {
  if (!isSSR) {
    return localStorage.getItem(key);
  } else {
    return undefined;
  }
};

const getDefaultSessionStorage = (key: string) => {
  if (!isSSR) {
    return sessionStorage.getItem(key);
  } else {
    return undefined;
  }
};

export function useStorage(
  key: string,
  defaultValue?: string
): [string | undefined, (arg0: string) => void, () => void] {
  const [storedValue, setStoredValue] = useState(
    getDefaultStorage(key) || defaultValue
  );

  const setStorageValue = (value: string) => {
    if (!isSSR) {
      localStorage.setItem(key, value);
      if (value !== storedValue) {
        setStoredValue(value);
      }
    }
  };

  const removeStorage = () => {
    if (!isSSR) {
      localStorage.removeItem(key);
    }
  };

  useEffect(() => {
    const storageValue = localStorage.getItem(key);
    if (storageValue) {
      setStoredValue(storageValue);
    }
  }, []);

  return [storedValue, setStorageValue, removeStorage];
}



export function useSessionStorage(
  key: string,
  defaultValue?: string
): [string | undefined, (arg0: string) => void, () => void] {
  const [storedValue, setStoredValue] = useState(
    getDefaultSessionStorage(key) || defaultValue
  );

  const setStorageValue = (value: string) => {
    if (!isSSR) {
      sessionStorage.setItem(key, value);
      if (value !== storedValue) {
        setStoredValue(value);
      }
    }
  };

  const removeStorage = () => {
    if (!isSSR) {
      sessionStorage.removeItem("key");
    }
  };

  useEffect(() => {
    const storageValue = sessionStorage.getItem(key);
    if (storageValue) {
      setStoredValue(storageValue);
    }
  }, []);

  return [storedValue, setStorageValue, removeStorage];
}
