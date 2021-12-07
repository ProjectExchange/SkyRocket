/**
 * backend
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 0.1.0
 * 
 *
 * NOTE: This class is auto generated by the swagger code generator program.
 * https://github.com/swagger-api/swagger-codegen.git
 * Do not edit the class manually.
 */
import { Gender } from './gender';
import { Role } from './role';

export interface AuthUser { 
    id: number;
    firstname: string;
    lastname: string;
    email: string;
    birthday: string;
    gender: Gender;
    roles: Array<Role>;
}