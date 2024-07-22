import {createApp} from "vue";
import App from "./App.vue";
import router from './router';
import 'ant-design-vue/dist/reset.css';
import Antd from 'ant-design-vue';
import VMdPreview from '@kangc/v-md-editor/lib/preview';
import '@kangc/v-md-editor/lib/style/preview.css';
import githubTheme from '@kangc/v-md-editor/lib/theme/github.js';
import '@kangc/v-md-editor/lib/theme/style/github.css';
import hljs from 'highlight.js';

VMdPreview.use(githubTheme, {
    Hljs: hljs,
});

router.beforeEach((to, from, next) => {
    if (to.meta.title) {
        document.title = to.meta.title
    }
    next()
})

createApp(App)
    .use(VMdPreview)
    .use(Antd)
    .use(router)
    .mount("#app");
