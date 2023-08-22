function changeTheme(theme: string) {
  if (theme === "dark") {
    document.body.setAttribute("theme-mode", "dark");
  } else {
    document.body.removeAttribute("theme-mode");
  }
}

export default changeTheme;
