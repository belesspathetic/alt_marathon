## 🪇 Alt Marathon
**Alt Marathon** is a versatile script designed to automate the process of content creation and publishing on a Facebook page.

### 🔧 Setup
To get started with Alt Marathon, follow these steps to set up your environment:
1. **Facebook API Key**:
 - Obtain your Facebook API key from the Facebook Developer portal.
 - Set your API key in an environment variable named ACCESS_TOKEN.
```cmd
setx ACCESS_TOKEN "key"
```
2. **Facebook Page ID**:
 - Locate your Facebook page ID.
 - Set your page ID in an environment variable named PAGE_ID.
```cmd
setx PAGE_ID "id"
```
3. **Chromedriver Path**:
 - Download the appropriate chromedriver.exe for your system.
 - Add the path to your chromedriver.exe to the PATH environment variable.

4. **FFmpeg Installation**:
 - Install FFmpeg on your system to handle video processing and encoding.

5. **Custom Fonts**:
 - Place your desired font file (e.g., font.ttf) in the root directory of the project.


### 🚀 Features
 - **API Key Security:** Utilizes environment variables to securely store and access your Facebook API key.
 - **Custom Fonts:** Supports custom fonts for unique and branded content presentation.
 - **Media Handling:** Automatically integrates images and videos into your posts, optimizing them for Facebook's platform.

### 🛣️ Roadmap
☑️ - Automated feed posts.
☑️ - Video cropping.
☑️ - Video speed up.
☑️ - Manual flag.
[ ] - Automated reels posts.