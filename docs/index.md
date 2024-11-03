---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "STARK demo"
  tagline: STARK - Redefining trust.
  actions:
    - theme: brand
      text: Docs
      link: /introduction
  image:
    src: /logo-large.jpg
    alt: VitePress
features:
  - title: Tính minh bạch
    details: STARK sử dụng các thuật toán thường dễ hiểu hơn và minh bạch hơn so với các zk-SNARKs khác, tạo ra sự tin tưởng lớn hơn vào hệ thống.
  - title: Khả năng mở rộng
    details: STARK tạo ra các bằng chứng có kích thước nhỏ hơn đáng kể, giảm chi phí lưu trữ và truyền tải. Ngoài ra, quá trình xác minh cho các bằng chứng STARK thường nhanh hơn nhiều, khiến nó phù hợp cho các ứng dụng hiệu suất cao.
  - title: Tính linh hoạt
    details: STARK có thể được sử dụng để chứng minh một loạt các phép tính, bao gồm cả những phép tính phức tạp. Hơn nữa, nó cung cấp mức độ tùy chỉnh cao để phù hợp với các yêu cầu ứng dụng cụ thể.
  - title: Quyền riêng tư
    details: Tương tự như các zk-SNARKs khác, STARK cho phép người chứng minh thể hiện tính đúng đắn của một tuyên bố mà không tiết lộ bất kỳ thông tin nào ngoài tính hợp lệ của tuyên bố đó.
  - title: Độ tin cậy
    details: STARK có sự phụ thuộc thấp hơn vào các tham số thiết lập đáng tin cậy, giảm các rủi ro liên quan đến việc các tham số này bị xâm phạm.
  - title: Khả năng chống lại máy tính lượng tử
    details: Một trong những ưu điểm đáng chú ý nhất của STARK là khả năng chống lại các cuộc tấn công từ máy tính lượng tử. Điều này làm cho STARK trở thành một giải pháp bảo mật hướng tới tương lai hơn so với các công nghệ khác.
---

<style>
:root {
  --vp-home-hero-name-color: transparent;
  --vp-home-hero-name-background: -webkit-linear-gradient(120deg, #53B6CC 30%, #E16413);

  --vp-home-hero-image-background-image: linear-gradient(-45deg, #53B6CC 50%, #E16413 50%);
  --vp-home-hero-image-filter: blur(44px);
}

@media (min-width: 640px) {
  :root {
    --vp-home-hero-image-filter: blur(56px);
  }
}

@media (min-width: 960px) {
  :root {
    --vp-home-hero-image-filter: blur(68px);
  }
}
</style>
