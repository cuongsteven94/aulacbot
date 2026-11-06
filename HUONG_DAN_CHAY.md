# AuLacBot — Hướng dẫn chạy

Source này là bản AuLacBot hoàn chỉnh ở tầng brand: fork từ CopilotKit OpenDuet (MIT),
gói brand riêng ở `openbot/examples/aulacbot`, đã chọn sẵn trong `.env` qua
`TENANT_PACKAGE_DIR=../examples/aulacbot`. Xem `AULACBOT.md` để biết các quyết định
(brand, ngôn ngữ, vì sao giữ CopilotKit Intelligence).

## Cần gì trước khi chạy

1. Một máy có **Docker**, **Bun 1.3+**, **Node** (để chạy `npx`).
2. **CopilotKit Intelligence API Key** của project AuLacBot.
3. **OpenAI API Key** (model mặc định), hoặc key Anthropic nếu đổi provider trong
   `openbot/examples/aulacbot/model.yaml`.

## Các bước

Chạy mọi lệnh từ thư mục gốc của source này.

```bash
# 1. Điền 2 key vào file .env (file đã có sẵn trong source, đừng commit file này):
#    INTELLIGENCE_API_KEY=<key Intelligence của project AuLacBot>
#    OPENAI_API_KEY=<key OpenAI của bạn>
#    (INTELLIGENCE_API_URL và INTELLIGENCE_GATEWAY_WS_URL đã trỏ sẵn về dịch vụ
#     managed của CopilotKit, giữ nguyên.)

# 2. Cài dependencies
bun install

# 3. Tạo Learning container "openbot" cho project và chép key vào đúng chỗ
bun openbot/scripts/setup-learning.ts

# 4. Khởi động (Postgres + máy tính của Bot chạy bằng Docker, API + app tự lên)
bash scripts/start.sh

# 5. Mở trình duyệt
#    http://localhost:3010
```

Dừng app: `bash scripts/stop.sh`.

## Kiểm tra nhanh sau khi lên

- Sidebar / màn hình đăng nhập hiện tên **AuLacBot**.
- Bot chính tên **AuLacBot**, trò chuyện mặc định bằng tiếng Việt, thuật ngữ kỹ thuật
  (Bot, Skill, Thread, API Key…) giữ nguyên tiếng Anh.
- Menu `/` hiện tên Skill bằng tiếng Việt (Brief buổi sáng, Trả lời email…).

## Lấy update từ upstream sau này

```bash
git fetch upstream
git checkout -b sync/upstream-<ngay>
git merge upstream/main
# test ổn rồi mới merge nhánh sync này vào nhánh aulacbot
```

Đừng sửa trực tiếp vào code trong `openbot/` ngoài gói `examples/aulacbot` — giữ core
sạch thì mỗi lần sync mới nhẹ.
