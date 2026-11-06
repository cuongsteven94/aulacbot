<div align="center">

# AuLacBot

### Trợ lý AI chủ động — tự làm việc, và luôn để bạn quyết định bước quan trọng

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

</div>

## AuLacBot là gì?

AuLacBot là một trợ lý AI chủ động dành cho người Việt. AuLacBot có máy tính riêng để tự làm việc: đọc email, xem lịch, tổng hợp việc cần xử lý mỗi sáng, research trên web, soạn sẵn nội dung — và **luôn hỏi ý bạn trước** khi gửi, thanh toán, ký hay xóa bất cứ thứ gì.

Giao diện và trải nghiệm được Việt hóa, trong khi các thuật ngữ kỹ thuật (agent, thread, workflow, API key, token, skill, prompt...) được giữ nguyên tiếng Anh cho chính xác.

## Tính năng chính

- **Brief buổi sáng** — tổng hợp việc cần bạn xử lý và việc có thể chờ, từ Gmail / Calendar / Slack
- **Làm việc tự chủ có kiểm soát** — Bot tự làm trên máy tính riêng của nó, mọi hành động quan trọng đều qua approval của bạn
- **Đa nền tảng** — dùng trên web, app desktop (Windows / macOS), app mobile (iOS / Android), Slack và Microsoft Teams
- **Memory cá nhân** — Bot ghi nhớ ngữ cảnh làm việc của bạn qua các thread hội thoại
- **Skills** — bộ kỹ năng sẵn có: trả lời email, research web công khai, tìm tài liệu, kiểm tra khẳng định với nguồn...

## Cài đặt

**File cài sẵn:** vào trang [Releases](../../releases) của repo này, mở bản phát hành mới nhất và tải ở mục Assets — không cần tài khoản GitHub:

- **Windows:** file `.exe` (bộ cài AuLacBot cho máy tính)
- **Android:** file `AuLacBot.apk` (app điều khiển trên điện thoại)

Mỗi bản phát hành được GitHub Actions build tự động từ source của đúng phiên bản đó.

**Chạy từ source:** xem hướng dẫn chi tiết trong [HUONG_DAN_CHAY.md](./HUONG_DAN_CHAY.md).

## Nền tảng kỹ thuật

AuLacBot được phát triển dựa trên mã nguồn mở [OpenDuet](https://github.com/CopilotKit/OpenDuet) và [OpenBot](https://github.com/CopilotKit/openbot) của CopilotKit (giấy phép MIT). Phần lõi được giữ nguyên để nhận cập nhật từ upstream; toàn bộ phần thương hiệu và gói cấu hình riêng của AuLacBot nằm ở thư mục `openbot/examples/aulacbot` và được chọn qua biến `TENANT_PACKAGE_DIR`.

Thread và memory của Bot chạy trên CopilotKit Intelligence. Model AI có thể thay đổi linh hoạt (OpenAI hoặc endpoint tương thích OpenAI, hoặc Anthropic) qua file cấu hình `model.yaml`.

## Giấy phép

MIT — xem file [LICENSE](./LICENSE). AuLacBot giữ nguyên thông tin bản quyền của CopilotKit trong mã nguồn theo đúng điều kiện của giấy phép MIT.
