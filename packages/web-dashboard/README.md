# Security Nexus - Web Dashboard

Web interface for the Polkadot Security Nexus platform.

## Features

- Real-time security monitoring dashboard
- SAFT Enhanced static analysis interface
- Alert management and notifications
- Chain status monitoring
- Privacy layer integration
- Responsive design with TailwindCSS

## Tech Stack

- **Framework**: Next.js 14 (App Router)
- **UI**: React 18, TailwindCSS
- **Charts**: Recharts
- **Icons**: Lucide React
- **State Management**: TanStack Query

## Getting Started

### Prerequisites

- Node.js 18+
- pnpm 8+

### Installation

```bash
pnpm install
```

### Development

```bash
pnpm dev
```

Open [http://localhost:3000](http://localhost:3000) in your browser.

### Build

```bash
pnpm build
pnpm start
```

## Environment Variables

Create a `.env.local` file:

```env
API_URL=http://localhost:3001
NEXT_PUBLIC_API_URL=http://localhost:3001
NEXT_PUBLIC_WS_URL=ws://localhost:3001
```

## Project Structure

```
src/
├── app/              # Next.js app router pages
│   ├── layout.tsx    # Root layout
│   ├── page.tsx      # Dashboard home
│   ├── analysis/     # Static analysis page
│   └── monitoring/   # Monitoring page
├── components/       # Reusable components
└── lib/              # Utilities and helpers
```

## Features Overview

### Dashboard
- Security metrics overview
- Active alerts panel
- Recent analysis results
- Chain status monitoring

### Static Analysis
- Upload pallet files
- Run security scans
- View detailed reports
- Export in multiple formats

### Monitoring
- Real-time transaction monitoring
- Attack pattern detection
- Detector status
- Alert history

## License

MIT
