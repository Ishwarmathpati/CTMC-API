<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get cart by id in business unit</name>
   <tag></tag>
   <elementGuidId>87624695-f78f-4ac5-af74-58ffe3bc2dda</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer {{ctp_access_token}}</value>
      <webElementGuid>5006a5c7-54da-432f-ac90-552e809f9410</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>ce1e4f1c-7419-465f-9879-628d63135a14</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${host}/${project-key}/as-associate/${associate-id}/in-business-unit/key=${business-unit-key}/carts/${cart-id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>bdb1fa53-15e2-485f-8e31-0bf265137ea5</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>dc014471-de30-41cd-95d0-9f3380665e06</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.associate-id</defaultValue>
      <description></description>
      <id>f0997ca0-6daf-4b6e-83be-a12c81599c2e</id>
      <masked>false</masked>
      <name>associate-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.business-unit-key</defaultValue>
      <description></description>
      <id>e0625d66-72bc-4a31-8e50-6e9f83078c59</id>
      <masked>false</masked>
      <name>business-unit-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart-id</defaultValue>
      <description></description>
      <id>c668ba61-82e6-4c59-9a07-94d17af91862</id>
      <masked>false</masked>
      <name>cart-id</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
