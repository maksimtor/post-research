import { NestFactory } from '@nestjs/core';
import { Transport } from '@nestjs/microservices';
import { join } from 'path';
import { AppModule } from './app.module';


async function bootstrap() {
  const app = await NestFactory.createMicroservice(AppModule, {
    transport: Transport.GRPC,
    options: {
      url: '0.0.0.0:50051',
      protoPath: join(__dirname, '../../../contracts/research_parser/research_parser.proto'),
      package: 'research_parser',
    },
  });

  app.listen();
}
bootstrap();
