<template>
  <div class="review">
    <h1>Review Pending Content</h1>
    <div v-for="content in pendingContent" :key="content.id" class="content-item">
      <h3>{{ content.title }}</h3>
      <p>{{ content.description }}</p>
      <button @click="approveContent(content.id)">Approve</button>
      <button @click="rejectContent(content.id)">Reject</button>
    </div>
  </div>
</template>

<script>
import axios from 'axios';

export default {
  data() {
    return {
      pendingContent: []
    };
  },
  created() {
    // Fetch pending content from the backend
    axios.get('/api/reviews/pending').then(response => {
      this.pendingContent = response.data;
    });
  },
  methods: {
    approveContent(contentId) {
      axios.post('/api/reviews/', {
        content_id: contentId,
        approved: true
      }).then(() => {
        this.pendingContent = this.pendingContent.filter(c => c.id !== contentId);
      });
    },
    rejectContent(contentId) {
      axios.post('/api/reviews/', {
        content_id: contentId,
        approved: false
      }).then(() => {
        this.pendingContent = this.pendingContent.filter(c => c.id !== contentId);
      });
    }
  }
};
</script>

<style>
.content-item {
  margin-bottom: 20px;
}
</style>
