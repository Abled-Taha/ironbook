/** @type {import('next-sitemap').IConfig} */
module.exports = {
  siteUrl: process.env.SITE_URL || 'https://iron-book-self.vercel.app',
  generateRobotsTxt: true, // (optional)
}
