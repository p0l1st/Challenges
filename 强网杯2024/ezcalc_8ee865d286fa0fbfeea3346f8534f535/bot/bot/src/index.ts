import express from 'express';
import { Request, Response, NextFunction } from 'express';
import { botHandler } from './bot';

const asyncHandler = func => (req: Request, res: Response, next: NextFunction) => {
    return Promise
        .resolve(func(req, res, next))
        .catch(next);
};

const main = async () => {

    const app = express();

    app.use(express.json());

    app.get('/api/bot', asyncHandler(botHandler));

    app.use((err: any, req: any, res: any, next: any) => {
        if (err) {
            console.error(`[-] Error processing request ${(req as any).id}: ${err}`);
            console.error(err.stack);
            res.status(500).json({ status: 'error', message: 'Internal server error', data: { id: (req as any).id } });
        } else {
            next();
        }
    });

    app.on('close', () => {
        process.exit(0);
    });

    app.listen(52000, () => {
        console.log('[+] Bot server started');
    });
}

main();
