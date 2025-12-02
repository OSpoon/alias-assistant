import { ref, onMounted, onBeforeUnmount } from 'vue';

export function useTheme() {
  const currentTheme = ref<string>('slate');
  
  const themes = [
    { name: 'neutral', label: 'Neutral' },
    { name: 'zinc', label: 'Zinc' },
    { name: 'gray', label: 'Gray' },
    { name: 'slate', label: 'Slate' },
  ];

  function setTheme(theme: string) {
    currentTheme.value = theme;
    const html = document.documentElement;
    html.removeAttribute('data-theme');
    html.setAttribute('data-theme', theme);
    localStorage.setItem('alias-assistant-theme', theme);
    updateDarkMode();
  }

  let mediaQuery: MediaQueryList | null = null;

  function updateDarkMode() {
    const html = document.documentElement;
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    
    if (prefersDark) {
      html.classList.add('dark');
    } else {
      html.classList.remove('dark');
    }
  }

  function loadTheme() {
    const savedTheme = localStorage.getItem('alias-assistant-theme') || 'slate';
    setTheme(savedTheme);
  }

  onMounted(() => {
    loadTheme();
    updateDarkMode();
    mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    mediaQuery.addEventListener('change', updateDarkMode);
  });

  onBeforeUnmount(() => {
    if (mediaQuery) {
      mediaQuery.removeEventListener('change', updateDarkMode);
    }
  });

  return {
    themes,
    currentTheme,
    setTheme,
  };
}

