<template>
  <main>
    <GithubCorner />
    <Header />
    <router-view />
    <Footer />
  </main>
</template>

<script lang="ts">
import "@/scss/base.scss";

import { Options, Vue } from "vue-class-component";

import GithubCorner from "@/components/GithubCorner.vue";
import Footer from "@/components/Footer.vue";
import Header from "@/components/Header.vue";

@Options({
  components: { Footer, GithubCorner, Header },
})
export default class App extends Vue {}
</script>
