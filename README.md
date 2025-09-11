# reviewkub-service

#### Folder structure
```
actix-api/
├─ Cargo.toml
├─ .env                         # ENV สำหรับ dev
├─ .env.example                 # ตัวอย่างค่า ENV
├─ migrations/                  # ถ้าใช้ sqlx/sea-orm
│  └─ <timestamp>_init.sql
├─ Dockerfile
├─ Makefile                     # คำสั่งช่วย run/dev/test
├─ scripts/
│  └─ dev.sh                    # สคริปต์รัน dev
└─ src/
   ├─ main.rs                   # entrypoint: bootstrap server
   ├─ lib.rs                    # re-exports / glue (ถ้าต้องการ)
   ├─ config/                   # โหลด/validate config
   │  └─ mod.rs
   ├─ db/                       # การเชื่อมต่อฐานข้อมูล
   │  └─ mod.rs
   ├─ domain/                   # กฎธุรกิจ แบบไม่ผูก framework
   │  ├─ models.rs              # struct ของ domain
   │  └─ services.rs            # pure business logic
   ├─ infra/                    # ส่วนที่คุยกับโลกภายนอก
   │  ├─ repositories.rs        # impl storage (e.g. Postgres)
   │  └─ external_clients.rs    # เรียก service อื่นๆ ถ้ามี
   ├─ web/                      # ชั้นเว็บ (framework-specific)
   │  ├─ mod.rs                 # ฟังก์ชัน configure() รวมทุก scope
   │  ├─ routes.rs              # กำหนดเส้นทาง/Scope/Resource
   │  ├─ handlers/              # ตัวจัดการ HTTP (Adapter)
   │  │  ├─ health.rs
   │  │  └─ users.rs
   │  ├─ extractors.rs          # custom extractors / auth
   │  └─ middleware.rs          # middleware เฉพาะระบบ
   ├─ error.rs                  # AppError / Result alias
   ├─ telemetry.rs              # tracing/log/metrics
   └─ utils.rs                  # helpers เล็กๆ ที่ reusable
tests/
└─ integration_users.rs         # integration tests (actix test server)
```

#### command diesel

schema generate
```
diesel print-schema > src/schema.rs
```