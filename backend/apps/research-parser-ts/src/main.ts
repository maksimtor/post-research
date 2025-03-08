import { NestFactory } from '@nestjs/core';
import { Transport, MicroserviceOptions } from '@nestjs/microservices';
import { join } from 'path';
import { Logger } from '@nestjs/common';
import { grpcClientOptions } from './grpc-client.options';
import { AppModule } from './app.module';


async function bootstrap() {
  // const app = await NestFactory.create(AppModule);
  // app.connectMicroservice<MicroserviceOptions>(grpcClientOptions);

  // await app.startAllMicroservices();
  // await app.listen(3001);
  // console.log(`Application is running on: ${await app.getUrl()}`);
  const app = await NestFactory.createMicroservice(AppModule, {
    transport: Transport.GRPC,
    options: {
      // url: 'http://localhost:6379',
      url: '0.0.0.0:50051',
      protoPath: join(__dirname, '../../../contracts/research_parser/research_parser.proto'),
      package: 'research_parser',
    },
  });

  app.listen();
}
bootstrap();
