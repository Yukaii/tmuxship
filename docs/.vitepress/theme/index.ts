import DefaultTheme from 'vitepress/theme';
import type { Theme } from 'vitepress';
import ThemePreviewer from './components/ThemePreviewer.vue';
import './custom.css';

export default {
  extends: DefaultTheme,
  enhanceApp({ app }) {
    app.component('ThemePreviewer', ThemePreviewer);
  }
} satisfies Theme;
