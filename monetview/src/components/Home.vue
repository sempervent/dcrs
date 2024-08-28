<template>
  <div class="home">
    <h1>Welcome to MonetView</h1>
    <section>
      <h2>Trending Posts</h2>
      <div class="gallery">
        <img v-for="post in trendingPosts" :src="post.image" :alt="post.title" :key="post.id" />
      </div>
    </section>
    <section>
      <h2>Recent Posts</h2>
      <div class="gallery">
        <img v-for="post in recentPosts" :src="post.image" :alt="post.title" :key="post.id" />
      </div>
    </section>
    <section>
      <h2>Your Posts</h2>
      <div class="gallery">
        <img v-for="post in userPosts" :src="post.image" :alt="post.title" :key="post.id" />
      </div>
    </section>
  </div>
</template>

<script>
import axios from 'axios';

export default {
  data() {
    return {
      trendingPosts: [],
      recentPosts: [],
      userPosts: []
    };
  },
  created() {
    // Fetch posts from the backend
    axios.get('/api/posts/trending').then(response => {
      this.trendingPosts = response.data;
    });
    axios.get('/api/posts/recent').then(response => {
      this.recentPosts = response.data;
    });
    axios.get('/api/posts/user').then(response => {
      this.userPosts = response.data;
    });
  }
};
</script>

<style>
.gallery {
  display: flex;
  flex-wrap: wrap;
}
.gallery img {
  margin: 10px;
  max-width: 150px;
  max-height: 150px;
  object-fit: cover;
}
</style>
