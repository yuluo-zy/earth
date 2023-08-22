import { GlobalContext } from '@/context';
import { useStorage } from '@/utils/useStorage.ts';
import { useEffect } from 'react';
import changeTheme from '@/utils/changeTheme.ts';
import { BrowserRouter, Route, Routes } from 'react-router-dom';
import LayoutPage from '@/layout.tsx';


function App() {
    const [theme, setTheme] = useStorage("theme", "light");
    useEffect(() => {
        changeTheme(theme as string);
    }, [theme]);

    const contextValue = {
        theme,
        setTheme,
    };

  return (
      <BrowserRouter>
      <GlobalContext.Provider value={contextValue}>
          <Routes>
              <Route path={'/'} element={<LayoutPage />} />
          </Routes>
      </GlobalContext.Provider>
      </BrowserRouter>
  );
}

export default App;
